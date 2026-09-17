# Spec: Secure by Construction

本規格是撰寫時安全基線。人類與 coding agent 開任何 OpenSpec change 時都必須引用它。這不是事後 code review 流程。

每個 change 對下列每一款標「適用」或「不適用」加原因。適用款至少寫一個描述**本變更**的 GIVEN／WHEN／THEN，不要只複誦本檔原文。

### Requirement: 信任邊界與誠實能力
每個 OpenSpec change SHALL 陳述信任邊界，以及本變更不提供的安全能力。

#### Scenario: Proposal lists who talks to what
- **GIVEN** 一個 OpenSpec change
- **WHEN** 作者或 agent 寫 `proposal.md` 或 `design.md`
- **THEN** 必須列出：誰輸入、誰執行、資料存在哪、有沒有後端／帳號／機密
- **AND** 沒有的控制不得寫成已做（例如無登入就不得宣稱已防未授權存取）

### Requirement: 託管與發布路徑一致
文件與 CI 宣稱的發布來源 SHALL 等於實際託管來源。

#### Scenario: Declared publish path is the real Pages source
- **GIVEN** 專案有靜態網站或自動部署
- **WHEN** 規格描述 push 某分支即更新線上
- **THEN** 必須寫明實際來源（例如 GitHub Pages 的 `gh-pages` 分支）
- **AND** workflow 必須寫入該來源，而不是寫一個平台會拒絕的另一條路徑
- **AND** 線上驗證列為 tasks，未驗證不得勾完成

### Requirement: 第三方腳本不得無完整性載入
瀏覽器執行的第三方 JS／CSS SHALL 自託管或具 Subresource Integrity，且預設不得在執行期載入公開 CDN。

#### Scenario: Player page scripts come from same origin
- **GIVEN** 網頁玩家或文件站需要終端／UI 函式庫
- **WHEN** 撰寫 `index.html` 或打包 `dist/`
- **THEN** 腳本與樣式來自同來源路徑（例如 `./vendor/`）
- **AND** 不得以 jsDelivr 或 unpkg 等 CDN 作為執行期載入點
- **AND** Content-Security-Policy 不得為了圖方便而允許任意 `https:` 的 `script-src`

### Requirement: 平台做不到的控制不得當成已生效
安全標頭、CSP、frame 限制 SHALL 寫在實際會套用的機制上，並說明平台限制。

#### Scenario: GitHub Pages does not honor Netlify headers
- **GIVEN** 託管於 GitHub Pages
- **WHEN** 需要 CSP、referrer 政策或防嵌入
- **THEN** 使用 Pages 實際會送出的方法（文件或 meta；能接受的限制要寫清楚）
- **AND** 不得把 Netlify 或 Cloudflare 專用檔（例如 `_headers`）放進只給 GitHub Pages 的產物並宣稱已有安全標頭
- **AND** 不得宣稱已設定 `X-Frame-Options` 若平台根本送不出該標頭

### Requirement: 機密不得進倉
原始碼、工作日誌、Issue、PR SHALL NOT 含金鑰、token 字面值、密碼、`.env`、私鑰、或內部未核准報告。

#### Scenario: Public push contains only already-public identifiers
- **GIVEN** 任何 commit、PR 說明或公開文件
- **WHEN** 內容將被 push 到公開遠端
- **THEN** 只能出現已公開識別資訊（例如 GitHub 帳號、專案 URL、模型名稱）
- **AND** Actions 只引用 `${{ secrets.* }}`，不寫入權杖本體
- **AND** 內部審查全文留在維護者本機，不進公開倉

### Requirement: 輸入與資源有界
進入引擎、WASM 或 CI 的外部輸入 SHALL 有上限與結束語意。

#### Scenario: Input surfaces declare allow-list or limits
- **GIVEN** 鍵盤、觸控、query、檔案或環境變數會進入程式
- **WHEN** 規格描述該輸入面
- **THEN** 必須規定允許集合或長度／尺寸上限
- **AND** 按下與放開（或等價結束）語意若影響狀態，必須寫出
- **AND** 不得假設「這只是小遊戲所以不必限」而不寫理由

### Requirement: 不適用要寫下來
本 change 碰不到的基線條款 SHALL 在 `design.md` 用「不適用」加原因列出。

#### Scenario: Copy-only change still declares N/A hosting rules
- **GIVEN** 一個只改終端文案、不碰網頁與 CI 的 change
- **WHEN** 作者填安全場景
- **THEN** 「託管與發布路徑一致」「第三方腳本不得無完整性載入」「平台做不到的控制不得當成已生效」可標不適用並說明無靜態託管變更
- **AND** 「信任邊界與誠實能力」與「機密不得進倉」仍適用
