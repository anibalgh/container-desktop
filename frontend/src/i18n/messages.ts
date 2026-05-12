import type { Language } from "../lib/types";

export interface Messages {
  app: {
    failedToLoadSettings: string;
  };
  sidebar: {
    productNamePrimary: string;
    productNameSecondary: string;
    screens: Record<
      | "dashboard"
      | "containers"
      | "images"
      | "volumes"
      | "networks"
      | "compose"
      | "settings"
      | "about",
      string
    >;
    connected: string;
    disconnected: string;
  };
  statusBar: {
    version: string;
  };
  common: {
    save: string;
    saved: string;
    dismiss: string;
    refresh: string;
    cancel: string;
    create: string;
    creating: string;
    remove: string;
    connect: string;
    load: string;
    loading: string;
    notAvailable: string;
  };
  dashboard: {
    connecting: string;
    productName: string;
    daemonUnreachable: string;
    daemonHelp: string;
    title: string;
    stats: {
      containersRunning: string;
      containersStopped: string;
      images: string;
      architecture: string;
    };
    systemInformation: string;
    dockerVersion: string;
    os: string;
    architecture: string;
    endpoint: string;
  };
  settings: {
    title: string;
    failedToLoad: string;
    sections: {
      language: string;
      theme: string;
      dockerEndpoint: string;
      fontSize: string;
      monospaceFont: string;
    };
    language: {
      auto: string;
      english: string;
      spanish: string;
      currentAuto: (languageLabel: string) => string;
    };
    theme: {
      auto: string;
      manual: string;
    };
    dockerEndpointHint: string;
    fontSizes: {
      normal: string;
      large: string;
      larger: string;
    };
    systemDefault: string;
  };
  about: {
    title: string;
    subtitle: string;
    description: string;
    licenseTitle: string;
    licenseContent: string;
    technologyTitle: string;
    technologyContent: string;
    vibeCodingTitle: string;
    vibeCodingBody: string;
    footer: string;
  };
  compose: {
    title: string;
    filePathLabel: string;
    filePathPlaceholder: string;
    up: string;
    down: string;
    emptyState: string;
    downCompleted: string;
  };
  containers: {
    title: string;
    count: (count: number) => string;
    columns: {
      name: string;
      image: string;
      state: string;
      ports: string;
      created: string;
      actions: string;
    };
    empty: string;
    actions: {
      start: string;
      stop: string;
      restart: string;
      remove: string;
    };
    tabs: {
      logs: string;
      terminal: string;
      stats: string;
    };
    states: Record<
      | "Running"
      | "Exited"
      | "Paused"
      | "Restarting"
      | "Created"
      | "Removing"
      | "Dead",
      string
    >;
    logs: {
      tailPlaceholder: string;
      follow: string;
      empty: string;
    };
    terminal: {
      root: string;
      interactive: string;
      command: string;
      commandPlaceholder: string;
      empty: string;
      inputPlaceholder: string;
    };
    stats: {
      refresh: string;
      empty: string;
      cpu: string;
      memory: string;
      netRx: string;
      netTx: string;
      blockRead: string;
      blockWrite: string;
      pids: string;
    };
    confirmRemove: {
      title: string;
      message: (name: string) => string;
    };
  };
  images: {
    title: string;
    count: (count: number) => string;
    pull: string;
    columns: {
      repository: string;
      tag: string;
      imageId: string;
      size: string;
      created: string;
      actions: string;
    };
    empty: string;
    none: string;
    pullDialog: {
      title: string;
      imageName: string;
      imageNamePlaceholder: string;
      tag: string;
      tagPlaceholder: string;
      pull: string;
      pulling: string;
    };
    confirmRemove: {
      title: string;
      message: (name: string) => string;
    };
  };
  volumes: {
    title: string;
    count: (count: number) => string;
    columns: {
      name: string;
      driver: string;
      mountpoint: string;
      created: string;
      actions: string;
    };
    empty: string;
    createDialog: {
      title: string;
      namePlaceholder: string;
    };
  };
  networks: {
    title: string;
    count: (count: number) => string;
    columns: {
      name: string;
      driver: string;
      scope: string;
      subnet: string;
      gateway: string;
      actions: string;
    };
    empty: string;
    createDialog: {
      title: string;
      namePlaceholder: string;
    };
  };
}

export const en: Messages = {
  app: {
    failedToLoadSettings: "Failed to load settings.",
  },
  sidebar: {
    productNamePrimary: "Container",
    productNameSecondary: "Desktop",
    screens: {
      dashboard: "Dashboard",
      containers: "Containers",
      images: "Images",
      volumes: "Volumes",
      networks: "Networks",
      compose: "Compose",
      settings: "Settings",
      about: "About",
    },
    connected: "Connected",
    disconnected: "Disconnected",
  },
  statusBar: {
    version: "Container Desktop v0.1.0",
  },
  common: {
    save: "Save",
    saved: "Saved ✓",
    dismiss: "Dismiss",
    refresh: "Refresh",
    cancel: "Cancel",
    create: "Create",
    creating: "Creating...",
    remove: "Remove",
    connect: "Connect",
    load: "Load",
    loading: "Loading...",
    notAvailable: "—",
  },
  dashboard: {
    connecting: "Connecting to Docker...",
    productName: "Container Desktop",
    daemonUnreachable: "Docker daemon is not reachable.",
    daemonHelp: "Start Docker or configure the endpoint in Settings.",
    title: "Dashboard",
    stats: {
      containersRunning: "Containers Running",
      containersStopped: "Containers Stopped",
      images: "Images",
      architecture: "Architecture",
    },
    systemInformation: "System Information",
    dockerVersion: "Docker Version",
    os: "OS",
    architecture: "Architecture",
    endpoint: "Endpoint",
  },
  settings: {
    title: "Settings",
    failedToLoad: "Failed to load settings.",
    sections: {
      language: "Language",
      theme: "Theme",
      dockerEndpoint: "Docker Endpoint",
      fontSize: "Font Size",
      monospaceFont: "Monospace Font",
    },
    language: {
      auto: "Auto (System)",
      english: "English",
      spanish: "Spanish",
      currentAuto: (languageLabel) => `Currently using: ${languageLabel}`,
    },
    theme: {
      auto: "Auto (OS)",
      manual: "Manual",
    },
    dockerEndpointHint: "unix:///var/run/docker.sock, tcp://192.168.1.10:2375",
    fontSizes: {
      normal: "Normal",
      large: "Large",
      larger: "Larger",
    },
    systemDefault: "System default",
  },
  about: {
    title: "About",
    subtitle:
      "A brief look at the intent, technical foundation, and build philosophy behind Container Desktop.",
    description:
      "Container Desktop is a cross-platform desktop application designed to manage containers, images, volumes, networks, and Docker workflows from a modern, clear, and efficient interface. Its goal is to provide a comfortable visual experience for tasks that normally require the terminal, without sacrificing power or control.",
    licenseTitle: "License",
    licenseContent: "Distributed under the MIT license.",
    technologyTitle: "Technology",
    technologyContent:
      "Built on a modern and robust stack for desktop, frontend, and native Docker access.",
    vibeCodingTitle: "Vibe coding",
    vibeCodingBody:
      "This application was created entirely through a vibe coding approach: without the developer manually writing a single line of code, but guided at every step by their judgment, experience, and deep knowledge of software development, interface design, technical analysis, and Docker and container ecosystems. The result is a tool that combines creative automation with high-level human technical direction.",
    footer: "DarkSiteX",
  },
  compose: {
    title: "Docker Compose",
    filePathLabel: "Compose File Path",
    filePathPlaceholder: "/path/to/docker-compose.yml",
    up: "Up",
    down: "Down",
    emptyState: "Enter a compose file path and click Up to start.",
    downCompleted: "Compose down completed.",
  },
  containers: {
    title: "Containers",
    count: (count) => `${count} container${count === 1 ? "" : "s"}`,
    columns: {
      name: "Name",
      image: "Image",
      state: "State",
      ports: "Ports",
      created: "Created",
      actions: "Actions",
    },
    empty: "No containers found.",
    actions: {
      start: "Start",
      stop: "Stop",
      restart: "Restart",
      remove: "Remove",
    },
    tabs: {
      logs: "Logs",
      terminal: "Terminal",
      stats: "Stats",
    },
    states: {
      Running: "Running",
      Exited: "Exited",
      Paused: "Paused",
      Restarting: "Restarting",
      Created: "Created",
      Removing: "Removing",
      Dead: "Dead",
    },
    logs: {
      tailPlaceholder: "Tail N",
      follow: "Follow",
      empty: "No log output yet. Click Load.",
    },
    terminal: {
      root: "root",
      interactive: "Interactive",
      command: "Command",
      commandPlaceholder: "ls -la /",
      empty: "Click Connect to start terminal.",
      inputPlaceholder: "Type a command...",
    },
    stats: {
      refresh: "Refresh Stats",
      empty: "Click Refresh to load stats.",
      cpu: "CPU",
      memory: "Memory",
      netRx: "Net RX",
      netTx: "Net TX",
      blockRead: "Block Read",
      blockWrite: "Block Write",
      pids: "PIDs",
    },
    confirmRemove: {
      title: "Remove Container",
      message: (name) => `Remove ${name}? This cannot be undone.`,
    },
  },
  images: {
    title: "Images",
    count: (count) => `${count} image${count === 1 ? "" : "s"}`,
    pull: "Pull",
    columns: {
      repository: "Repository",
      tag: "Tag",
      imageId: "Image ID",
      size: "Size",
      created: "Created",
      actions: "Actions",
    },
    empty: "No images found.",
    none: "<none>",
    pullDialog: {
      title: "Pull Image",
      imageName: "Image Name",
      imageNamePlaceholder: "nginx, alpine...",
      tag: "Tag",
      tagPlaceholder: "latest",
      pull: "Pull",
      pulling: "Pulling...",
    },
    confirmRemove: {
      title: "Remove Image",
      message: (name) => `Remove ${name}?`,
    },
  },
  volumes: {
    title: "Volumes",
    count: (count) => `${count} volume${count === 1 ? "" : "s"}`,
    columns: {
      name: "Name",
      driver: "Driver",
      mountpoint: "Mountpoint",
      created: "Created",
      actions: "Actions",
    },
    empty: "No volumes found.",
    createDialog: {
      title: "Create Volume",
      namePlaceholder: "Volume name",
    },
  },
  networks: {
    title: "Networks",
    count: (count) => `${count} network${count === 1 ? "" : "s"}`,
    columns: {
      name: "Name",
      driver: "Driver",
      scope: "Scope",
      subnet: "Subnet",
      gateway: "Gateway",
      actions: "Actions",
    },
    empty: "No networks found.",
    createDialog: {
      title: "Create Network",
      namePlaceholder: "Network name",
    },
  },
};

export const es: Messages = {
  app: {
    failedToLoadSettings: "No se pudo cargar la configuración.",
  },
  sidebar: {
    productNamePrimary: "Container",
    productNameSecondary: "Desktop",
    screens: {
      dashboard: "Panel Principal",
      containers: "Contenedores",
      images: "Imágenes",
      volumes: "Volúmenes",
      networks: "Redes",
      compose: "Compose",
      settings: "Configuración",
      about: "Acerca de",
    },
    connected: "Conectado",
    disconnected: "Desconectado",
  },
  statusBar: {
    version: "Container Desktop v0.1.0",
  },
  common: {
    save: "Guardar",
    saved: "Guardado ✓",
    dismiss: "Cerrar",
    refresh: "Actualizar",
    cancel: "Cancelar",
    create: "Crear",
    creating: "Creando...",
    remove: "Eliminar",
    connect: "Conectar",
    load: "Cargar",
    loading: "Cargando...",
    notAvailable: "—",
  },
  dashboard: {
    connecting: "Conectando con Docker...",
    productName: "Container Desktop",
    daemonUnreachable: "No se puede acceder al daemon de Docker.",
    daemonHelp: "Inicia Docker o configura el endpoint en Configuración.",
    title: "Panel Principal",
    stats: {
      containersRunning: "Contenedores en ejecución",
      containersStopped: "Contenedores detenidos",
      images: "Imágenes",
      architecture: "Arquitectura",
    },
    systemInformation: "Información del sistema",
    dockerVersion: "Versión de Docker",
    os: "Sistema operativo",
    architecture: "Arquitectura",
    endpoint: "Endpoint",
  },
  settings: {
    title: "Configuración",
    failedToLoad: "No se pudo cargar la configuración.",
    sections: {
      language: "Idioma",
      theme: "Tema",
      dockerEndpoint: "Endpoint de Docker",
      fontSize: "Tamaño de fuente",
      monospaceFont: "Fuente monoespaciada",
    },
    language: {
      auto: "Automático (Sistema)",
      english: "Inglés",
      spanish: "Español",
      currentAuto: (languageLabel) => `Usando actualmente: ${languageLabel}`,
    },
    theme: {
      auto: "Automático (SO)",
      manual: "Manual",
    },
    dockerEndpointHint: "unix:///var/run/docker.sock, tcp://192.168.1.10:2375",
    fontSizes: {
      normal: "Normal",
      large: "Grande",
      larger: "Más grande",
    },
    systemDefault: "Predeterminado del sistema",
  },
  about: {
    title: "Acerca de",
    subtitle:
      "Una vista breve sobre la intención, la base técnica y la filosofía de construcción de Container Desktop.",
    description:
      "Container Desktop es una aplicación de escritorio multiplataforma diseñada para administrar contenedores, imágenes, volúmenes, redes y flujos de trabajo con Docker desde una interfaz moderna, clara y eficiente. Su objetivo es ofrecer una experiencia visual cómoda para tareas que normalmente requieren consola, sin perder potencia ni control.",
    licenseTitle: "Licencia",
    licenseContent: "Distribuida bajo licencia MIT.",
    technologyTitle: "Tecnología",
    technologyContent:
      "Construida con una base moderna y robusta para escritorio, frontend y acceso nativo a Docker.",
    vibeCodingTitle: "Vibe coding",
    vibeCodingBody:
      "Esta aplicación ha sido creada íntegramente mediante un enfoque de vibe coding: sin escribir manualmente una sola línea de código por parte del desarrollador, pero guiada en todo momento por su criterio, experiencia y conocimiento profundo en desarrollo de software, diseño de interfaces, análisis técnico y ecosistemas Docker y contenedores. El resultado es una herramienta que combina automatización creativa con dirección técnica humana de alto nivel.",
    footer: "DarkSiteX",
  },
  compose: {
    title: "Docker Compose",
    filePathLabel: "Ruta del archivo Compose",
    filePathPlaceholder: "/ruta/al/docker-compose.yml",
    up: "Levantar",
    down: "Bajar",
    emptyState:
      "Ingresa la ruta de un archivo Compose y haz clic en Levantar para iniciar.",
    downCompleted: "Compose down completado.",
  },
  containers: {
    title: "Contenedores",
    count: (count) => `${count} contenedor${count === 1 ? "" : "es"}`,
    columns: {
      name: "Nombre",
      image: "Imagen",
      state: "Estado",
      ports: "Puertos",
      created: "Creado",
      actions: "Acciones",
    },
    empty: "No se encontraron contenedores.",
    actions: {
      start: "Iniciar",
      stop: "Detener",
      restart: "Reiniciar",
      remove: "Eliminar",
    },
    tabs: {
      logs: "Logs",
      terminal: "Terminal",
      stats: "Estadísticas",
    },
    states: {
      Running: "En ejecución",
      Exited: "Detenido",
      Paused: "Pausado",
      Restarting: "Reiniciando",
      Created: "Creado",
      Removing: "Eliminando",
      Dead: "Muerto",
    },
    logs: {
      tailPlaceholder: "Últimas N",
      follow: "Seguir",
      empty: "Todavía no hay salida de logs. Haz clic en Cargar.",
    },
    terminal: {
      root: "root",
      interactive: "Interactivo",
      command: "Comando",
      commandPlaceholder: "ls -la /",
      empty: "Haz clic en Conectar para iniciar la terminal.",
      inputPlaceholder: "Escribe un comando...",
    },
    stats: {
      refresh: "Actualizar estadísticas",
      empty: "Haz clic en Actualizar para cargar las estadísticas.",
      cpu: "CPU",
      memory: "Memoria",
      netRx: "Red RX",
      netTx: "Red TX",
      blockRead: "Lectura de bloque",
      blockWrite: "Escritura de bloque",
      pids: "PIDs",
    },
    confirmRemove: {
      title: "Eliminar contenedor",
      message: (name) => `¿Eliminar ${name}? Esta acción no se puede deshacer.`,
    },
  },
  images: {
    title: "Imágenes",
    count: (count) => `${count} imagen${count === 1 ? "" : "es"}`,
    pull: "Descargar",
    columns: {
      repository: "Repositorio",
      tag: "Tag",
      imageId: "ID de imagen",
      size: "Tamaño",
      created: "Creada",
      actions: "Acciones",
    },
    empty: "No se encontraron imágenes.",
    none: "<ninguna>",
    pullDialog: {
      title: "Descargar imagen",
      imageName: "Nombre de imagen",
      imageNamePlaceholder: "nginx, alpine...",
      tag: "Tag",
      tagPlaceholder: "latest",
      pull: "Descargar",
      pulling: "Descargando...",
    },
    confirmRemove: {
      title: "Eliminar imagen",
      message: (name) => `¿Eliminar ${name}?`,
    },
  },
  volumes: {
    title: "Volúmenes",
    count: (count) => `${count} volumen${count === 1 ? "" : "es"}`,
    columns: {
      name: "Nombre",
      driver: "Driver",
      mountpoint: "Punto de montaje",
      created: "Creado",
      actions: "Acciones",
    },
    empty: "No se encontraron volúmenes.",
    createDialog: {
      title: "Crear volumen",
      namePlaceholder: "Nombre del volumen",
    },
  },
  networks: {
    title: "Redes",
    count: (count) => `${count} red${count === 1 ? "" : "es"}`,
    columns: {
      name: "Nombre",
      driver: "Driver",
      scope: "Alcance",
      subnet: "Subred",
      gateway: "Puerta de enlace",
      actions: "Acciones",
    },
    empty: "No se encontraron redes.",
    createDialog: {
      title: "Crear red",
      namePlaceholder: "Nombre de la red",
    },
  },
};

export const messages: Record<Language, Messages> = {
  en,
  es,
};
