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
      | "security"
      | "volumes"
      | "networks"
      | "compose"
      | "settings"
      | "about",
      string
    >;
    cleanup: string;
    connected: string;
    disconnected: string;
  };
  statusBar: {
    version: (version: string) => string;
  };
  common: {
    save: string;
    saved: string;
    dismiss: string;
    refresh: string;
    open: string;
    cancel: string;
    create: string;
    creating: string;
    remove: string;
    connect: string;
    disconnect: string;
    copy: string;
    load: string;
    loading: string;
    notAvailable: string;
    search: string;
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
    security: {
      title: string;
      scannedImages: (count: number) => string;
      scannedImagesLabel: string;
      vulnerabilitiesBySeverity: string;
      countLabel: string;
      noResults: string;
    };
    cleanup: {
      title: string;
      reclaimableSpace: string;
      noGarbage: string;
      prune: string;
      pruning: string;
      confirmTitle: string;
      confirmMessage: string;
      cleanedAmount: (amount: string) => string;
    };
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
    dockerEndpointRemoteHelpLink: string;
    dockerEndpointRemoteHelp: {
      title: string;
      intro: string;
      installLabel: string;
      remoteCommandLabel: string;
      localCommandLabel: string;
      configStringLabel: string;
      installCommands: string[];
      remoteCommand: string;
      localCommand: string;
      configString: string;
    };
    remoteTcpWarning: {
      title: string;
      trustedNetwork: string;
      sshRecommendation: string;
      continueSave: string;
    };
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
    browse: string;
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
      copy: string;
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
      usage: string;
      size: string;
      created: string;
      actions: string;
    };
    empty: string;
    none: string;
    filterLabel: string;
    searchPlaceholder: string;
    selectionMode: string;
    exitSelectionMode: string;
    selectAllFiltered: string;
    clearSelection: string;
    selectedCount: (count: number) => string;
    removeSelected: string;
    inUse: (count: number) => string;
    unused: string;
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
      bulkTitle: string;
      bulkMessage: (count: number) => string;
    };
  };
  security: {
    title: string;
    subtitle: string;
    toolsTitle: string;
    imagesTitle: string;
    selectedCount: (count: number) => string;
    rescanSelected: string;
    rescanning: string;
    toolAvailable: (version: string) => string;
    toolUnavailable: string;
    installHint: string;
    emptyImages: string;
    noFindings: string;
    searchPlaceholder: string;
    summary: {
      totalImages: string;
      scannedImages: string;
      imagesWithFindings: string;
      severityChart: string;
    };
    columns: {
      image: string;
      findings: string;
      tools: string;
      lastScan: string;
    };
    toolState: Record<"Idle" | "Running" | "Completed" | "Failed", string>;
    detail: {
      title: (imageName: string) => string;
      emptyTitle: string;
      selectImage: string;
      noReports: string;
      findings: (count: number) => string;
      columns: {
        vulnerability: string;
        package: string;
        version: string;
        severity: string;
      };
      references: string;
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
    confirmRemove: {
      title: string;
      message: (name: string) => string;
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
    confirmRemove: {
      title: string;
      message: (name: string) => string;
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
      security: "Security",
      volumes: "Volumes",
      networks: "Networks",
      compose: "Compose",
      settings: "Settings",
      about: "About",
    },
    cleanup: "Cleanup",
    connected: "Connected",
    disconnected: "Disconnected",
  },
  statusBar: {
    version: (version) => `Container Desktop v${version}`,
  },
  common: {
    save: "Save",
    saved: "Saved ✓",
    dismiss: "Dismiss",
    refresh: "Refresh",
    open: "Open",
    cancel: "Cancel",
    create: "Create",
    creating: "Creating...",
    remove: "Remove",
    connect: "Connect",
    disconnect: "Disconnect",
    copy: "Copy",
    load: "Load",
    loading: "Loading...",
    notAvailable: "—",
    search: "Search",
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
    security: {
      title: "Security summary",
      scannedImages: (count) => `${count} scanned image${count === 1 ? "" : "s"}`,
      scannedImagesLabel: "Scanned images",
      vulnerabilitiesBySeverity: "Vulnerabilities by severity",
      countLabel: "Count",
      noResults: "No current security summary is available.",
    },
    cleanup: {
      title: "Docker cleanup",
      reclaimableSpace: "Reclaimable Docker garbage",
      noGarbage: "No reclaimable Docker garbage was detected.",
      prune: "Clean Docker",
      pruning: "Cleaning...",
      confirmTitle: "Clean Docker data",
      confirmMessage:
        "Run docker system prune now? This removes stopped containers, unused networks, dangling images, and build cache.",
      cleanedAmount: (amount) => `Cleanup completed. Freed ${amount}.`,
    },
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
    dockerEndpointHint: "Local example: unix:///var/run/docker.sock | Trusted LAN example: tcp://192.168.0.25:2375",
    dockerEndpointRemoteHelpLink: "Need remote connection instructions?",
    dockerEndpointRemoteHelp: {
      title: "Connect to a remote Docker host",
      intro: "Direct tcp:// connections to Docker daemons on a trusted local network are supported. For stronger protection, prefer an SSH tunnel with port forwarding so the Docker socket stays bound to remote loopback.",
      installLabel: "Install socat on the remote machine",
      remoteCommandLabel: "Run this on the remote machine",
      localCommandLabel: "Run this on your local machine",
      configStringLabel: "Use this value in Container Desktop",
      installCommands: [
        "Ubuntu / Debian: sudo apt update && sudo apt install -y socat",
        "Fedora: sudo dnf install -y socat",
        "Arch: sudo pacman -S socat",
        "macOS (Homebrew): brew install socat",
      ],
      remoteCommand: "socat TCP-LISTEN:2375,bind=127.0.0.1,fork UNIX-CONNECT:/var/run/docker.sock",
      localCommand: "ssh -N -L 23750:127.0.0.1:2375 usuario@192.168.0.135",
      configString: "tcp://127.0.0.1:23750",
    },
    remoteTcpWarning: {
      title: "Save trusted-LAN TCP endpoint?",
      trustedNetwork: "Only use direct tcp:// Docker endpoints on local networks you trust. Plain TCP does not provide transport security.",
      sshRecommendation: "For better security, prefer SSH with port forwarding and keep the remote Docker socket bound to loopback.",
      continueSave: "Save anyway",
    },
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
    browse: "Browse...",
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
      copy: "Copy output",
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
      usage: "Usage",
      size: "Size",
      created: "Created",
      actions: "Actions",
    },
    empty: "No images found.",
    none: "<none>",
    filterLabel: "Filter list:",
    searchPlaceholder: "Filter images by repository, tag, or ID",
    selectionMode: "Multi-select",
    exitSelectionMode: "Done selecting",
    selectAllFiltered: "Select visible",
    clearSelection: "Clear",
    selectedCount: (count) => `${count} selected`,
    removeSelected: "Remove selected",
    inUse: (count) => `Used by ${count} container${count === 1 ? "" : "s"}`,
    unused: "Unused",
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
      bulkTitle: "Remove images",
      bulkMessage: (count) => `Remove ${count} selected images?`,
    },
  },
  security: {
    title: "Security",
    subtitle:
      "Detect scanners, persist selected tools, and review consolidated vulnerability results for local images.",
    toolsTitle: "Scanner tools",
    imagesTitle: "Image security status",
    selectedCount: (count) => `${count} selected scanner${count === 1 ? "" : "s"}`,
    rescanSelected: "Run selected scanners",
    rescanning: "Scheduling scans...",
    toolAvailable: (version) => `Available · ${version}`,
    toolUnavailable: "Not installed on this system",
    installHint: "Click to view installation instructions for this OS.",
    emptyImages: "No Docker images are available to scan.",
    noFindings: "No findings",
    searchPlaceholder: "Filter images by name, tag, or ID",
    summary: {
      totalImages: "Total images",
      scannedImages: "Images with stored results",
      imagesWithFindings: "Images with findings",
      severityChart: "Unified findings by severity",
    },
    columns: {
      image: "Image",
      findings: "Findings",
      tools: "Tools",
      lastScan: "Last scan",
    },
    toolState: {
      Idle: "Idle",
      Running: "Running",
      Completed: "Completed",
      Failed: "Failed",
    },
    detail: {
      title: (imageName) => `Results for ${imageName}`,
      emptyTitle: "Image details",
      selectImage: "Select an image to inspect the stored vulnerability results.",
      noReports: "No stored reports were found for this image yet.",
      findings: (count) => `${count} finding${count === 1 ? "" : "s"}`,
      columns: {
        vulnerability: "Vulnerability",
        package: "Package",
        version: "Installed version",
        severity: "Severity",
      },
      references: "References",
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
    confirmRemove: {
      title: "Remove Volume",
      message: (name) => `Remove ${name}? This cannot be undone.`,
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
    confirmRemove: {
      title: "Remove Network",
      message: (name) => `Remove ${name}? This cannot be undone.`,
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
      security: "Seguridad",
      volumes: "Volúmenes",
      networks: "Redes",
      compose: "Compose",
      settings: "Configuración",
      about: "Acerca de",
    },
    cleanup: "Limpieza",
    connected: "Conectado",
    disconnected: "Desconectado",
  },
  statusBar: {
    version: (version) => `Container Desktop v${version}`,
  },
  common: {
    save: "Guardar",
    saved: "Guardado ✓",
    dismiss: "Cerrar",
    refresh: "Actualizar",
    open: "Abrir",
    cancel: "Cancelar",
    create: "Crear",
    creating: "Creando...",
    remove: "Eliminar",
    connect: "Conectar",
    disconnect: "Desconectar",
    copy: "Copiar",
    load: "Cargar",
    loading: "Cargando...",
    notAvailable: "—",
    search: "Buscar",
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
    security: {
      title: "Resumen de seguridad",
      scannedImages: (count) => `${count} imagen${count === 1 ? "" : "es"} escaneada${count === 1 ? "" : "s"}`,
      scannedImagesLabel: "Imágenes escaneadas",
      vulnerabilitiesBySeverity: "Vulnerabilidades por severidad",
      countLabel: "Cantidad",
      noResults: "No hay un resumen de seguridad disponible actualmente.",
    },
    cleanup: {
      title: "Limpieza de Docker",
      reclaimableSpace: "Basura Docker recuperable",
      noGarbage: "No se detectó basura Docker recuperable.",
      prune: "Limpiar Docker",
      pruning: "Limpiando...",
      confirmTitle: "Limpiar datos de Docker",
      confirmMessage:
        "¿Ejecutar docker system prune ahora? Esto elimina contenedores detenidos, redes sin uso, imágenes colgantes y caché de compilación.",
      cleanedAmount: (amount) => `Limpieza completada. Se liberaron ${amount}.`,
    },
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
    dockerEndpointHint: "Ejemplo local: unix:///var/run/docker.sock | Ejemplo LAN confiable: tcp://192.168.0.25:2375",
    dockerEndpointRemoteHelpLink: "¿Necesitas instrucciones para conexión remota?",
    dockerEndpointRemoteHelp: {
      title: "Conectarse a un Docker remoto",
      intro: "Las conexiones directas tcp:// a daemons Docker en una LAN de confianza están permitidas. Para mayor seguridad, prefiere un túnel SSH con port forwarding para que el socket Docker remoto permanezca ligado al loopback remoto.",
      installLabel: "Instala socat en la máquina remota",
      remoteCommandLabel: "Ejecuta esto en la máquina remota",
      localCommandLabel: "Ejecuta esto en tu máquina local",
      configStringLabel: "Usa este valor en Container Desktop",
      installCommands: [
        "Ubuntu / Debian: sudo apt update && sudo apt install -y socat",
        "Fedora: sudo dnf install -y socat",
        "Arch: sudo pacman -S socat",
        "macOS (Homebrew): brew install socat",
      ],
      remoteCommand: "socat TCP-LISTEN:2375,bind=127.0.0.1,fork UNIX-CONNECT:/var/run/docker.sock",
      localCommand: "ssh -N -L 23750:127.0.0.1:2375 usuario@192.168.0.135",
      configString: "tcp://127.0.0.1:23750",
    },
    remoteTcpWarning: {
      title: "¿Guardar endpoint TCP de LAN confiable?",
      trustedNetwork: "Usa endpoints Docker tcp:// directos solo en redes locales de confianza. TCP plano no ofrece seguridad de transporte.",
      sshRecommendation: "Para mayor seguridad, prefiere SSH con port forwarding y mantén el socket Docker remoto ligado al loopback.",
      continueSave: "Guardar de todos modos",
    },
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
    browse: "Buscar...",
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
      copy: "Copiar salida",
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
      usage: "Uso",
      size: "Tamaño",
      created: "Creada",
      actions: "Acciones",
    },
    empty: "No se encontraron imágenes.",
    none: "<ninguna>",
    filterLabel: "Filtro de la lista:",
    searchPlaceholder: "Filtra imágenes por repositorio, tag o ID",
    selectionMode: "Selección múltiple",
    exitSelectionMode: "Terminar selección",
    selectAllFiltered: "Seleccionar visibles",
    clearSelection: "Limpiar",
    selectedCount: (count) => `${count} seleccionada${count === 1 ? "" : "s"}`,
    removeSelected: "Eliminar seleccionadas",
    inUse: (count) => `Usada por ${count} contenedor${count === 1 ? "" : "es"}`,
    unused: "Sin uso",
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
      bulkTitle: "Eliminar imágenes",
      bulkMessage: (count) => `¿Eliminar ${count} imágenes seleccionadas?`,
    },
  },
  security: {
    title: "Seguridad",
    subtitle:
      "Detecta escáneres, persiste la selección del usuario y revisa resultados consolidados de vulnerabilidades para las imágenes locales.",
    toolsTitle: "Herramientas de análisis",
    imagesTitle: "Estado de seguridad por imagen",
    selectedCount: (count) => `${count} herramienta${count === 1 ? "" : "s"} seleccionada${count === 1 ? "" : "s"}`,
    rescanSelected: "Ejecutar herramientas seleccionadas",
    rescanning: "Programando análisis...",
    toolAvailable: (version) => `Disponible · ${version}`,
    toolUnavailable: "No está instalada en este sistema",
    installHint: "Haz clic para ver instrucciones de instalación para este sistema operativo.",
    emptyImages: "No hay imágenes Docker disponibles para analizar.",
    noFindings: "Sin hallazgos",
    searchPlaceholder: "Filtra imágenes por nombre, tag o ID",
    summary: {
      totalImages: "Imágenes totales",
      scannedImages: "Imágenes con resultados guardados",
      imagesWithFindings: "Imágenes con hallazgos",
      severityChart: "Hallazgos unificados por severidad",
    },
    columns: {
      image: "Imagen",
      findings: "Hallazgos",
      tools: "Herramientas",
      lastScan: "Último análisis",
    },
    toolState: {
      Idle: "En espera",
      Running: "Analizando",
      Completed: "Completado",
      Failed: "Falló",
    },
    detail: {
      title: (imageName) => `Resultados para ${imageName}`,
      emptyTitle: "Detalle de imagen",
      selectImage: "Selecciona una imagen para inspeccionar los resultados almacenados.",
      noReports: "Todavía no se encontraron reportes guardados para esta imagen.",
      findings: (count) => `${count} hallazgo${count === 1 ? "" : "s"}`,
      columns: {
        vulnerability: "Vulnerabilidad",
        package: "Paquete",
        version: "Versión instalada",
        severity: "Severidad",
      },
      references: "Referencias",
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
    confirmRemove: {
      title: "Eliminar volumen",
      message: (name) => `¿Eliminar ${name}? Esta acción no se puede deshacer.`,
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
    confirmRemove: {
      title: "Eliminar red",
      message: (name) => `¿Eliminar ${name}? Esta acción no se puede deshacer.`,
    },
  },
};

export const messages: Record<Language, Messages> = {
  en,
  es,
};
