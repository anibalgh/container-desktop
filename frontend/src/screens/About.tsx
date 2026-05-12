const STACK = [
  "Tauri v2",
  "Rust",
  "React",
  "TypeScript",
  "Tailwind CSS",
  "Bollard",
];

export function AboutScreen() {
  return (
    <div className="p-6 h-full overflow-auto">
      <div className="max-w-3xl space-y-6">
        <header className="space-y-2">
          <h1 className="text-2xl font-semibold" style={{ color: "var(--color-text)" }}>
            Acerca de
          </h1>
          <p className="text-sm leading-6" style={{ color: "var(--color-text-muted)" }}>
            Una vista breve sobre la intencion, la base tecnica y la filosofia de construccion de Container Desktop.
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
            Container Desktop es una aplicacion de escritorio multiplataforma disenada para administrar
            contenedores, imagenes, volumenes, redes y flujos de trabajo con Docker desde una interfaz
            moderna, clara y eficiente. Su objetivo es ofrecer una experiencia visual comoda para tareas
            que normalmente requieren consola, sin perder potencia ni control.
          </p>
          <div className="grid gap-4 md:grid-cols-2">
            <InfoBlock
              title="Licencia"
              content="Distribuida bajo licencia MIT."
            />
            <InfoBlock
              title="Tecnologia"
              content="Construida con una base moderna y robusta para escritorio, frontend y acceso nativo a Docker."
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
            Vibe coding
          </h2>
          <p className="leading-7" style={{ color: "var(--color-text)" }}>
            Esta aplicacion ha sido creada integramente mediante un enfoque de vibe coding: sin escribir
            manualmente una sola linea de codigo por parte del desarrollador, pero guiada en todo momento
            por su criterio, experiencia y conocimiento profundo en desarrollo de software, diseno de
            interfaces, analisis tecnico y ecosistemas Docker y contenedores. El resultado es una herramienta
            que combina automatizacion creativa con direccion tecnica humana de alto nivel.
          </p>
        </section>

        <footer className="pt-2">
          <p className="text-sm" style={{ color: "var(--color-text-muted)" }}>
            DarkSiteX
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
