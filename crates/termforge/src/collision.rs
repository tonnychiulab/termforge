use crate::ecs::EntityId;
use crate::math::{Rect, Vec2};

pub type CollisionLayer = u32;

/// Axis-Aligned Bounding Box collider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AABB {
    pub half_extents: Vec2,
}

impl AABB {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            half_extents: Vec2::new(width * 0.5, height * 0.5),
        }
    }

    pub fn to_rect(&self, center: Vec2) -> Rect {
        Rect::new(
            center.x - self.half_extents.x,
            center.y - self.half_extents.y,
            self.half_extents.x * 2.0,
            self.half_extents.y * 2.0,
        )
    }
}

/// Circular collider.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub const fn new(radius: f32) -> Self {
        Self { radius }
    }

    #[inline]
    pub fn intersects(&self, center: Vec2, other: &Circle, other_center: Vec2) -> bool {
        let total_radius = self.radius + other.radius;
        center.distance_squared(other_center) <= total_radius * total_radius
    }

    #[inline]
    pub fn intersects_aabb(&self, center: Vec2, aabb: &AABB, aabb_center: Vec2) -> bool {
        let rect = aabb.to_rect(aabb_center);
        // Find closest point on rect to circle center
        let closest_x = center.x.clamp(rect.position.x, rect.position.x + rect.size.x);
        let closest_y = center.y.clamp(rect.position.y, rect.position.y + rect.size.y);
        let closest_pt = Vec2::new(closest_x, closest_y);

        center.distance_squared(closest_pt) <= self.radius * self.radius
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CollisionResult {
    pub collided: bool,
    pub penetration: f32,
    pub normal: Vec2,
}

#[derive(Clone, Copy)]
pub struct QuadtreeItem {
    pub entity: EntityId,
    pub bounds: Rect,
    pub layer: CollisionLayer,
    pub mask: CollisionLayer,
}

/// 2D Quadtree Spatial Partitioning structure.
pub struct Quadtree {
    bounds: Rect,
    capacity: usize,
    items: Vec<QuadtreeItem>,
    children: Option<Box<[Quadtree; 4]>>,
}

impl Quadtree {
    pub fn new(bounds: Rect, capacity: usize) -> Self {
        Self {
            bounds,
            capacity,
            items: Vec::with_capacity(capacity),
            children: None,
        }
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.children = None;
    }

    pub fn insert(&mut self, item: QuadtreeItem) -> bool {
        if !self.bounds.intersects(item.bounds) {
            return false;
        }

        if self.children.is_none() && self.items.len() < self.capacity {
            self.items.push(item);
            return true;
        }

        if self.children.is_none() {
            self.subdivide();
        }

        if let Some(children) = &mut self.children {
            let mut inserted = false;
            for child in children.iter_mut() {
                if child.insert(item) {
                    inserted = true;
                }
            }
            inserted
        } else {
            false
        }
    }

    fn subdivide(&mut self) {
        let half_w = self.bounds.size.x * 0.5;
        let half_h = self.bounds.size.y * 0.5;
        let x = self.bounds.position.x;
        let y = self.bounds.position.y;

        let nw = Quadtree::new(Rect::new(x, y, half_w, half_h), self.capacity);
        let ne = Quadtree::new(Rect::new(x + half_w, y, half_w, half_h), self.capacity);
        let sw = Quadtree::new(Rect::new(x, y + half_h, half_w, half_h), self.capacity);
        let se = Quadtree::new(Rect::new(x + half_w, y + half_h, half_w, half_h), self.capacity);

        let mut children = Box::new([nw, ne, sw, se]);

        // Redistribute items
        let old_items = std::mem::take(&mut self.items);
        for item in old_items {
            for child in children.iter_mut() {
                child.insert(item);
            }
        }

        self.children = Some(children);
    }

    pub fn query_candidates(&self, bounds: Rect, results: &mut Vec<QuadtreeItem>) {
        if !self.bounds.intersects(bounds) {
            return;
        }

        for item in &self.items {
            if item.bounds.intersects(bounds) {
                results.push(*item);
            }
        }

        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_candidates(bounds, results);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_collision() {
        let c1 = Circle::new(5.0);
        let c2 = Circle::new(3.0);

        assert!(c1.intersects(Vec2::new(0.0, 0.0), &c2, Vec2::new(7.0, 0.0)));
        assert!(!c1.intersects(Vec2::new(0.0, 0.0), &c2, Vec2::new(9.0, 0.0)));
    }
}
