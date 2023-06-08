use std::{
    env,
    sync::{Arc, Mutex},
    thread::{self, spawn},
    time::Duration,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use log::{info, warn};
use sysinfo::{PidExt, ProcessExt, System, SystemExt, UserExt, Process};

struct Proceso {
    pid: u32,
    programa: String,
    inicio_ejecucion: DateTime<Utc>,
    segundos_en_ejecucion: u64,
    usuario: String,
}

impl Proceso {
    fn new(
        pid: u32,
        programa: String,
        inicio_ejecucion: DateTime<Utc>,
        segundos_en_ejecucion: u64,
        usuario: String,
    ) -> Proceso {
        Proceso {
            pid,
            programa,
            inicio_ejecucion,
            segundos_en_ejecucion,
            usuario,
        }
    }
}

fn mostrar_info_proceso(proceso: &Proceso) {
    println!("PID: {}", proceso.pid);
    println!("Programa: {}", proceso.programa);
    println!("Inicio de la ejecución: {}", proceso.inicio_ejecucion);
    println!("Segundos en ejecución: {}", proceso.segundos_en_ejecucion);
    println!("Usuario: {}", proceso.usuario);
}

fn mostrar_mensaje_info(texto: &str) {
    info!("Info: {}", texto);
    println!("{}", texto);
}

fn mostrar_mensaje_argumento_segundos_no_indicado() {
    mostrar_mensaje_info("Segundos entre muestra de procesos ejecutados no indicados como argumento, tomaremos 60 segundos por defecto.");
}

fn mostrar_mensaje_argumento_nombre_programa_no_indicado() {
    mostrar_mensaje_info("Nombre de programa para filtrar los procesos no indicado como argumento, no se filtrará por nombre de programa.");
}

fn mostrar_mensaje_argumento_nombre_usuario_no_indicado() {
    mostrar_mensaje_info("Nombre de usuario para filtrar los procesos no indicado como argumento, no se filtrará por usuario.");
}

fn esta_el_proceso_dentro_del_intervalo_de_tiempo(proceso: &Process, inicio: u64, fin: u64) -> bool {
    let tiempo_proceso = proceso.start_time() + proceso.run_time();
    tiempo_proceso >= inicio && tiempo_proceso <= fin
}

fn mensaje_warning(texto: &str) {
    warn!("Warning: {}", texto);
    println!("{}", texto);
}

fn mensaje_warning_problema_al_obtener_usuario_del_proceso(nombre_proceso: &str) {
    mensaje_warning(format!("Hubo un problema obteniendo la información del usuario que lanzó la aplicación con nombre: {}", nombre_proceso).as_str());
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let procesos: Arc<Mutex<Vec<Proceso>>> = Arc::new(Mutex::new(Vec::new()));

    let procesos_copia_1 = Arc::clone(&procesos);

    let segundos_intervalo: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

    let segundos_intervalo_copia_1 = Arc::clone(&segundos_intervalo);

    spawn(move || {
        let segundos = match args.len() {
            2..=4 => match args[1].parse::<u64>() {
                Ok(secs) => secs,
                Err(_) => {
                    mostrar_mensaje_argumento_segundos_no_indicado();
                    60
                },
            }
            _ => {
                mostrar_mensaje_argumento_segundos_no_indicado();
                60
            },
        };

        let program_name = match args.len() {
            3..=4 => match args[2].as_str() {
                "-" => {
                    mostrar_mensaje_argumento_nombre_programa_no_indicado();
                    None
                },
                _ => Some(args[2].as_str()),
            }
            _ => {
                mostrar_mensaje_argumento_nombre_programa_no_indicado();
                None
            },
        };

        let username = match args.len() {
            4 => match args[3].as_str() {
                "-" => {
                    mostrar_mensaje_argumento_nombre_usuario_no_indicado();
                    None
                },
                _ => Some(args[3].as_str()),
            }
            _ => {
                mostrar_mensaje_argumento_nombre_usuario_no_indicado();
                None
            },
        };

        *segundos_intervalo_copia_1.lock().unwrap() = segundos;

        loop {
            thread::sleep(Duration::from_secs(segundos));

            println!("Lista de programas en ejecución - {}", Utc::now());

            let mut procesos_temp = procesos_copia_1.lock().unwrap();

            for proceso in procesos_temp.iter() {
                let mut mostrar = false;

                if let Some(nombre_programa) = program_name {
                    if proceso.programa.contains(nombre_programa) {
                        if let Some(nombre_usuario) = username {
                            if proceso.usuario.contains(nombre_usuario) {
                                mostrar = true;
                            }
                        }
                        else {
                            mostrar = true;
                        }
                    }
                }
                else {
                    mostrar = true;
                }

                if mostrar {
                    mostrar_info_proceso(proceso);
                    println!();
                }
            }

            procesos_temp.clear();
        }
    });

    let procesos_copia_2 = Arc::clone(&procesos);

    loop {
        let sistema = System::new_all();

        let segundos = *segundos_intervalo.lock().unwrap();

        if segundos > 0 {
            let ahora = u64::try_from(Utc::now().timestamp()).unwrap();
    
            let fin = ahora + segundos;
            
            for datos_proceso in sistema.processes() {
                if esta_el_proceso_dentro_del_intervalo_de_tiempo(datos_proceso.1, ahora, fin) {
                    let mut procesos_temp = procesos_copia_2.lock().unwrap();
    
                    let mut puesto = Option::None;
        
                    for (indice, proceso) in procesos_temp.iter().enumerate() {
                        if proceso.pid.eq(&datos_proceso.0.as_u32()) {
                            puesto = Some(indice);
                        }
                    }
        
                    if let Some(indice) = puesto {
                        procesos_temp.remove(indice);
                    }
        
                    let pid = datos_proceso.0.as_u32();
                    let programa = datos_proceso.1.name().to_string();
                    let inicio_ejecucion: DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp_opt(i64::try_from(datos_proceso.1.start_time()).unwrap(),0,).unwrap(),Utc);
                    let segundos_en_ejecucion = datos_proceso.1.run_time();
                    let usuario = match datos_proceso.1.user_id() {
                        Some(id) => match sistema.get_user_by_id(id) {
                            Some(user) => user.name().to_string(),
                            None => {
                                mensaje_warning_problema_al_obtener_usuario_del_proceso(programa.as_str());
                                "".to_string()
                            },
                        },
                        None => {
                            mensaje_warning_problema_al_obtener_usuario_del_proceso(programa.as_str());
                            "".to_string()
                        },
                    };
        
                    procesos_temp.push(Proceso::new(pid,programa,inicio_ejecucion,segundos_en_ejecucion,usuario));
                }
            }
        }
    }
}
