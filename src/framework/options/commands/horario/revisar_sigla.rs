use ::serenity::{builder::{CreateEmbed, CreateEmbedFooter}, model::Timestamp};
use crate::{utils::components::responses, Context, Error};
use BCUCRust::cursos::*;
use poise::serenity_prelude as serenity;

#[derive(poise::ChoiceParameter)]
pub enum Periodos {
    #[name = "2023-1"]
    Periodo1 = 1,
    #[name = "2023-2"]
    Periodo2 = 2,
    #[name = "2023-3"]
    Periodo3 = 3,
    #[name = "2024-1"]
    Periodo4 = 4,

}

impl From<Periodos> for String {
    fn from(periodo: Periodos) -> Self {
        match periodo {
            Periodos::Periodo1 => "2023-1".to_string(),
            Periodos::Periodo2 => "2023-2".to_string(),
            Periodos::Periodo3 => "2023-3".to_string(),
            Periodos::Periodo4 => "2024-1".to_string(),
        }
    }
}

#[poise::command(
    slash_command,
    prefix_command,
    category = "Universidad",
    required_bot_permissions = "SEND_MESSAGES",
    user_cooldown = 5,
)]
/// Eliges el periodo y nombre de algún curso para ver todos los cursos de ese tipo
pub async fn revisar_sigla(
    ctx: Context<'_>,
    #[description = "Periodo a seleccionar"] periodo: Periodos,
    #[description = "Sigla del curso"] sigla: String,
) -> Result<(), Error> {
    let sigla_string: String = sigla.replace(" ", "+");
    let periodo_string: String = periodo.into();
    let cursos_busqueda = buscar_sigla(&periodo_string, &sigla_string).await;
    let mut embeds: Vec<CreateEmbed> = vec![];
    let username = &ctx.author().name;
    let userurl = ctx.author().avatar_url().expect("none").to_string();

    ctx.defer().await?;

    match cursos_busqueda {
        Ok(cursos) => {

            for curso in cursos {
                let horarios = &curso.horario;
                let horarios_string: String = horarios.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
                embeds.push(
                    CreateEmbed::new()
                    .title("Resultados de la búsqueda")
                    .fields(vec![
                    ("NRC", curso.nrc.to_string(), true),
                    ("Sigla", curso.sigla, true),
                    ("Permite Retiro", curso.permite_retiro.to_string(), true),
                    ("Inglés", curso.ingles.to_string(), true),
                    ("Seccion", curso.seccion.to_string(), true),
                    ("Aprobación Especial", curso.aprobacion_especial.to_string(), true),
                    ("Área", curso.area, true),
                    ("Formato", curso.formato, true),
                    ("Categoría", curso.categoria, true),
                    ("Nombre", curso.nombre, true),
                    ("Profesor", curso.profesor, true),
                    ("Campus", curso.campus, true),
                    ("Creditos", curso.creditos.to_string(), true),
                    ("Vacantes Totales", curso.vacantes_totales.to_string(), true),
                    ("Vacantes Disponibles", curso.vacantes_disponibles.to_string(), true),
                    ("Horarios", horarios_string, true)
                    ])
                    .footer(CreateEmbedFooter::new(format!("Pedido por: {}", &username)).icon_url(&userurl))
                    .timestamp(Timestamp::now())
                    .color(serenity::Color::from_rgb(229, 189, 20))
                );
            }

        responses::paginate_embeds(ctx, embeds).await?;

        }

        Err(err) => {
            println!("Error en la busqueda de cursos: {}", err)
        }
    }

    Ok(())

}