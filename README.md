# File Converter Hub

A small, dependency-free Rust CLI for safe local file conversions. It focuses on conversions that can be implemented reliably without uploading files to a third party.

**Author:** Radwan Abdulhadi Ahmed · رضوان عبدالهادي أحمد · GitHub: @rad03i2

## Why it exists

Quick conversions should not require sending private documents to an online service. File Converter Hub runs locally, has no telemetry or network code, and refuses destructive overwrites by default.

## Features

- CSV → TSV and TSV → CSV with quoted-field and escaped-quote handling.
- Text newline normalization to LF or CRLF.
- Base64 encoding and decoding, including binary input/output.
- Atomic-style output through a temporary file and rename.
- Existing outputs are protected unless `--overwrite` is explicitly supplied.
- UTF-8 validation for text-based conversions.
- Zero runtime dependencies beyond the Rust standard library.

## Requirements & installation

Requires Rust 1.70+ (stable recommended).

```bash
git clone https://github.com/rad03i2/file-converter-hub.git
cd file-converter-hub
cargo build --release
```

The binary is `target/release/file-converter` (`.exe` on Windows). You can also run commands with `cargo run --`.

## Usage

```bash
file-converter csv-to-tsv contacts.csv contacts.tsv
file-converter tsv-to-csv data.tsv data.csv
file-converter normalize-text notes.txt notes-lf.txt
file-converter normalize-text notes.txt notes-win.txt --crlf
file-converter base64-encode photo.png photo.b64
file-converter base64-decode photo.b64 restored.png
```

Add `--no-final-newline` to text normalization when needed. Add `--overwrite` only when replacing an existing output is intentional. `--version` prints version and author metadata.

### Preview guidance

This is a terminal application. For a repository screenshot, show `--version`, one conversion command, and a small before/after CSV/TSV example; no GUI is claimed.

## Configuration

No environment variables, accounts, API keys, or configuration files are required.

## Project structure

```text
src/lib.rs          conversion engine and safe output writer
src/main.rs         CLI argument handling
tests/conversion.rs integration tests
.github/workflows/  cross-platform CI
```

## Testing

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI runs formatting, Clippy, tests, and release builds. Local commands above are the source of truth for reproducing validation.

## Security & privacy

Conversions are entirely local. The tool never uploads files. Existing outputs are not replaced without `--overwrite`. Input and output paths must differ at the CLI level. See [SECURITY.md](SECURITY.md) for vulnerability reporting.

## Limitations

This project deliberately does not claim universal document conversion. It currently supports CSV/TSV, newline normalization, and Base64. CSV parsing supports conventional RFC-4180-style quoting but does not preserve original formatting byte-for-byte. It does not convert PDF, Office, image, audio, or video formats. Replacing an existing output is not a transactional rollback operation if the final filesystem rename itself fails after removal on platforms that require removal first.

## Optional roadmap

Future work may add streaming for very large files and additional dependency-free formats where correctness can be maintained. Format support will only be documented after implementation and tests exist.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Keep conversions deterministic, local-first, and covered by tests.

## License

MIT — see [LICENSE](LICENSE).

## Author

**Radwan Abdulhadi Ahmed**  
**رضوان عبدالهادي أحمد**  
GitHub: **@rad03i2**

---

# العربية

## نظرة عامة

**File Converter Hub** أداة سطر أوامر مكتوبة بلغة Rust لتحويل مجموعة عملية من الملفات محليًا وبشكل آمن، من دون رفع ملفات المستخدم إلى أي خدمة خارجية.

## لماذا هذا المشروع؟

عمليات التحويل البسيطة لا ينبغي أن تتطلب إرسال ملفات خاصة إلى مواقع الإنترنت. تعمل الأداة محليًا بالكامل، ولا تحتوي على تتبع أو اتصالات شبكية، كما تحمي الملفات الموجودة من الاستبدال غير المقصود.

## المزايا

- تحويل CSV إلى TSV والعكس مع دعم الحقول المقتبسة وعلامات الاقتباس المهربة.
- توحيد نهايات الأسطر إلى LF أو CRLF.
- ترميز Base64 وفكّه مع دعم البيانات الثنائية.
- كتابة الناتج عبر ملف مؤقت ثم إعادة تسميته.
- منع استبدال ملف ناتج موجود افتراضيًا إلا عند تمرير `--overwrite`.
- التحقق من UTF-8 في التحويلات النصية.
- لا توجد تبعيات تشغيل خارج مكتبة Rust القياسية.

## المتطلبات والتثبيت

تحتاج Rust 1.70 أو أحدث.

```bash
git clone https://github.com/rad03i2/file-converter-hub.git
cd file-converter-hub
cargo build --release
```

## الاستخدام

```bash
file-converter csv-to-tsv contacts.csv contacts.tsv
file-converter tsv-to-csv data.tsv data.csv
file-converter normalize-text notes.txt notes-lf.txt
file-converter normalize-text notes.txt notes-win.txt --crlf
file-converter base64-encode photo.png photo.b64
file-converter base64-decode photo.b64 restored.png
```

استخدم `--no-final-newline` عند عدم الرغبة بسطر نهائي، و`--overwrite` فقط عندما تريد استبدال الناتج الموجود عمدًا.

## الإعداد

لا تحتاج الأداة إلى متغيرات بيئة أو حسابات أو مفاتيح API أو ملف إعدادات.

## بنية المشروع

- `src/lib.rs`: محرك التحويل والكتابة الآمنة.
- `src/main.rs`: واجهة سطر الأوامر.
- `tests/conversion.rs`: اختبارات الوظائف الأساسية.
- `.github/workflows/`: التكامل المستمر متعدد الأنظمة.

## الاختبارات

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## الأمان والخصوصية

كل العمليات محلية ولا يتم رفع أي ملف. الاستبدال ممنوع افتراضيًا، كما يجب أن يختلف مسار الإدخال عن الإخراج. راجع [SECURITY.md](SECURITY.md) للإبلاغ عن الثغرات.

## القيود

الأداة ليست محولًا شاملًا لكل الصيغ. الدعم الحالي هو CSV/TSV وتوحيد نهايات الأسطر وBase64 فقط. لا يوجد حاليًا تحويل PDF أو Office أو الصور أو الصوت أو الفيديو. محلل CSV يدعم الاقتباس التقليدي لكنه لا يضمن الحفاظ على تنسيق الملف الأصلي بايتًا ببايت.

## التطوير المستقبلي الاختياري

يمكن لاحقًا إضافة معالجة تدفقية للملفات الكبيرة وصيغ إضافية عندما يمكن دعمها بصورة صحيحة ومختبرة.

## المساهمة

راجع [CONTRIBUTING.md](CONTRIBUTING.md). يجب أن تكون التحويلات حتمية ومحلية ومغطاة بالاختبارات.

## الترخيص

MIT — راجع [LICENSE](LICENSE).

## المؤلف

**Radwan Abdulhadi Ahmed**  
**رضوان عبدالهادي أحمد**  
GitHub: **@rad03i2**
