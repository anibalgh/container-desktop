import { useI18n } from "../i18n";

const STACK = [
  "Tauri v2",
  "Rust",
  "React",
  "TypeScript",
  "Tailwind CSS",
  "Bollard",
];

export function AboutScreen() {
  const { t } = useI18n();

  return (
    <div className="p-6 h-full overflow-auto">
      <div className="max-w-3xl space-y-6">
        <header className="space-y-2">
          <h1 className="text-2xl font-semibold" style={{ color: "var(--color-text)" }}>
            {t.about.title}
          </h1>
          <p className="text-sm leading-6" style={{ color: "var(--color-text-muted)" }}>
            {t.about.subtitle}
          </p>
        </header>

        <section
          className="rounded-xl border p-6 space-y-4"
          style={{
            borderColor: "var(--color-border)",
            backgroundColor: "var(--color-surface-secondary)",
          }}
        >
          <p className="leading-7" style={{ color: "var(--color-text)" }}>
            {t.about.description}
          </p>
          <div className="grid gap-4 md:grid-cols-2">
            <InfoBlock
              title={t.about.licenseTitle}
              content={t.about.licenseContent}
            />
            <InfoBlock
              title={t.about.technologyTitle}
              content={t.about.technologyContent}
            />
          </div>
          <div className="flex flex-wrap gap-2">
            {STACK.map((item) => (
              <span
                key={item}
                className="px-3 py-1 text-xs rounded-full border"
                style={{
                  borderColor: "var(--color-border)",
                  color: "var(--color-text)",
                  backgroundColor: "var(--color-surface)",
                }}
              >
                {item}
              </span>
            ))}
          </div>
        </section>

        <section
          className="rounded-xl border p-6"
          style={{
            borderColor: "var(--color-border)",
            background:
              "linear-gradient(135deg, color-mix(in srgb, var(--color-accent) 16%, transparent), transparent 55%)",
          }}
        >
          <h2 className="text-sm font-semibold uppercase tracking-[0.2em] mb-3" style={{ color: "var(--color-text-muted)" }}>
            {t.about.vibeCodingTitle}
          </h2>
          <p className="leading-7" style={{ color: "var(--color-text)" }}>
            {t.about.vibeCodingBody}
          </p>
        </section>

        <footer className="pt-2">
          <p className="text-sm" style={{ color: "var(--color-text-muted)" }}>
            {t.about.footer}
          </p>
        </footer>
      </div>
    </div>
  );
}

function InfoBlock({ title, content }: { title: string; content: string }) {
  return (
    <div className="rounded-lg border p-4" style={{ borderColor: "var(--color-border)" }}>
      <h2 className="text-xs font-semibold uppercase tracking-[0.18em] mb-2" style={{ color: "var(--color-text-muted)" }}>
        {title}
      </h2>
      <p className="text-sm leading-6" style={{ color: "var(--color-text)" }}>
        {content}
      </p>
    </div>
  );
}
