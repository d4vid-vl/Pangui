use ::serenity::{builder::{CreateEmbed, CreateEmbedFooter}, model::Timestamp};
use crate::{Context, Error};
use BCUCRust::cursos::*;
use poise::{serenity_prelude as serenity, CreateReply};

#[derive(poise::ChoiceParameter)]
pub enum Periodos {
    #[name = "2024-1"]
    Periodo1 = 1,
    #[name = "2024-2"]
    Periodo2 = 2,
    #[name = "2025-1"]
    Periodo3 = 3,
    #[name = "2025-2"]
    Periodo4 = 4,
}

impl From<Periodos> for String {
    fn from(periodo: Periodos) -> Self {
        match periodo {
            Periodos::Periodo1 => "2024-1".to_string(),
            Periodos::Periodo2 => "2024-2".to_string(),
            Periodos::Periodo3 => "2025-1".to_string(),
            Periodos::Periodo4 => "2025-2".to_string(),
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
pub async fn revisar_nrc(
    ctx: Context<'_>,
    #[description = "Periodo a seleccionar"] periodo: Periodos,
    #[description = "NRC del curso"] nrc: i32,
) -> Result<(), Error> {
    ctx.defer().await?;

    let periodo_string: String = periodo.into();
    let curso = obtener_curso(&periodo_string, nrc).await?;
    let username = &ctx.author().name;
    let userurl = ctx.author().avatar_url().expect("none").to_string();
    let embed = {
        let horarios = &curso.horario;
        let horarios_string: String = horarios.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ");
            CreateEmbed::new()
            .title("Resultados de la búsqueda")
            .fields(vec![
            ("NRC", curso.nrc.to_string(), true),
            ("Sigla", curso.sigla.clone(), true),
            ("Permite Retiro", curso.permite_retiro.to_string(), true),
            ("Inglés", curso.ingles.to_string(), true),
            ("Seccion", curso.seccion.to_string(), true),
            ("Aprobación Especial", curso.aprobacion_especial.to_string(), true),
            ("Área", curso.area.clone(), true),
            ("Formato", curso.formato.clone(), true),
            ("Categoría", curso.categoria.clone(), true),
            ("Nombre", curso.nombre.clone(), true),
            ("Profesor", curso.profesor.clone(), true),
            ("Campus", curso.campus.clone(), true),
            ("Creditos", curso.creditos.to_string(), true),
            ("Vacantes Totales", curso.vacantes_totales.to_string(), true),
            ("Vacantes Disponibles", curso.vacantes_disponibles.to_string(), true),
            ("Horarios", horarios_string, true)
            ])
            .footer(CreateEmbedFooter::new(format!("Pedido por: {}", &username)).icon_url(&userurl))
            .timestamp(Timestamp::now())
            .color(serenity::Color::from_rgb(229, 189, 20))
            .thumbnail("https://cdn.discordapp.com/attachments/1203428555933876254/1208177205293883413/PUC.png?ex=65e255b4&is=65cfe0b4&hm=b9885400e96dbaede3fc2514b54251d7178da3460478ae415502b6bc7f4ed4b8&")
        };     
    let reply = CreateReply::default()
    .embed(embed);

    ctx.send(reply).await?;

    Ok(())               
}