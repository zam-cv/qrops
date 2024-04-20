using UnityEngine;

public class Ajustes : MonoBehaviour
{
    public GameObject panelConfiguracion; // Arrastra aquí tu panel de configuración en el Inspector
    private bool estaAbierto = false; // Variable para rastrear si el panel está abierto o no

    // Esta función se llama cuando se hace clic en el botón
    public void ToggleConfiguracion()
    {
        estaAbierto = !estaAbierto; // Invierte el estado actual

        if (estaAbierto)
        {
            panelConfiguracion.SetActive(true); // Activa el panel
            Time.timeScale = 0; // Pausa el juego
        }
        else
        {
            panelConfiguracion.SetActive(false); // Desactiva el panel
            Time.timeScale = 1; // Reanuda el juego
        }
    }
}
