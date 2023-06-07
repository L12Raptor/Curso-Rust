use std::{
    env,
    sync::{Arc, Mutex},
    thread::{self, spawn},
    time::Duration,
};

use chrono::{DateTime, NaiveDateTime, Utc};
use sysinfo::{PidExt, ProcessExt, System, SystemExt, UserExt};

fn main() {
    let args: Vec<String> = env::args().collect();

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

    let procesos: Arc<Mutex<Vec<Proceso>>> = Arc::new(Mutex::new(Vec::new()));

    let procesos_copia_1 = Arc::clone(&procesos);

    spawn(move || {
        let segundos = match args.len() {
            2..=4 => match args[1].parse::<u64>() {
                Ok(secs) => secs,
                Err(_) => {
                    println!("Segundos entre muestra de procesos ejecutados no indicados como argumento, tomaremos 60 segundos por defecto.");
                    60
                },
            }
            _ => {
                println!("Segundos entre muestra de procesos ejecutados no indicados como argumento, tomaremos 60 segundos por defecto.");
                60
            },
        };

        let program_name = match args.len() {
            3..=4 => match args[2].as_str() {
                "-" => {
                    println!("Nombre de programa para filtrar los procesos no indicado como argumento, no se filtrará por nombre de programa.");
                    None
                },
                _ => Some(args[2].as_str()),
            }
            _ => {
                println!("Nombre de programa para filtrar los procesos no indicado como argumento, no se filtrará por nombre de programa.");
                None
            },
        };

        let username = match args.len() {
            4 => match args[3].as_str() {
                "-" => {
                    println!("Nombre de usuario para filtrar los procesos no indicado como argumento, no se filtrará por usuario.");
                    None
                },
                _ => Some(args[3].as_str()),
            }
            _ => {
                println!("Nombre de usuario para filtrar los procesos no indicado como argumento, no se filtrará por usuario.");
                None
            },
        };

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
                    println!("PID: {}", proceso.pid);
                    println!("Programa: {}", proceso.programa);
                    println!("Inicio de la ejecución: {}", proceso.inicio_ejecucion);
                    println!("Segundos en ejecución: {}", proceso.segundos_en_ejecucion);
                    println!("Usuario: {}", proceso.usuario);
                    println!();
                }
            }

            procesos_temp.clear();
        }
    });

    let procesos_copia_2 = Arc::clone(&procesos);

    loop {
        let sistema = System::new_all();

        let ahora = Utc::now().timestamp();

        for datos_proceso in sistema.processes() {
            if (datos_proceso.1.start_time() + datos_proceso.1.run_time()) >= u64::try_from(ahora).unwrap() {
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
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                };
    
                procesos_temp.push(Proceso::new(pid,programa,inicio_ejecucion,segundos_en_ejecucion,usuario));
            }
        }
    }
}
