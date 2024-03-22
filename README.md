# Pangui
> Un bot para ayudar en asuntos de la Universidad Católica de Chile hecho en Rust.

[![Rust](https://img.shields.io/badge/Rust-%2B1.77-orange?logo=rust)](https://blog.rust-lang.org/2024/02/08/Rust-1.76.0.html)
[![License](https://img.shields.io/badge/License-MIT-red)](https://opensource.org/licenses/MIT)

Este bot principalmente será un proyecto más una mezcla entre utilidad para la Universidad y un bot multiusos para aprender a usar API's, librerias, etc...

## Comandos
<div align="center">
  <table style="width:100%">
    <tr>
      <td><em>Universidad</em></td>
      <td>
        <ul>
          <li>revisar_nombre: Busca cursos de la Universidad según el nombre entregado</li>
          <li>revisar_nrc: Busca un curso de la Universidad según el número de curso entregado</li>
          <li>revisar_profesor: Busca cursos de la Universidad según el nombre del profesor entregado</li>
          <li>revisar_sigla: Busca cursos de la Universidad según la sigla entregada</li>
        </ul>
      </td>
    </tr>
    <tr>
      <td><em>Spotify</em></td>
      <td>
        <ul>
          <li>spotify_lp: Entrega detalles en un embed sobre el álbum buscado</li>
          <li>spotify_artist: Entrega detalles en un embed sobre el artista buscado</li>
        </ul>
      </td>
    </tr>
    <tr>
      <td><em>Info</em></td>
      <td>
        <ul>
          <li>info: Da información general sobre el bot</li>
        </ul>
      </td>
    </tr>
  </table>
</div>

### Usar el Código

Para usar este código necesitaremos de un archivo .env en nuestra carpeta, la cual contenga:
  - DISCORD_TOKEN: El token de su bot de Discord
  - SPOTIFY_CLIENT: La ID de una aplicación de Spotify, de lo contrario los comandos de Spotify no funcionarán
  - SPOTIFY_SECRET: El token de la aplicación de Spotify

Luego de esto podremos ejecutar el bot de forma normal con "cargo run".