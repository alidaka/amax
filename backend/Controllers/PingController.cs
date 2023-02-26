using Microsoft.AspNetCore.Mvc;

namespace amax.Controllers;

[ApiController]
[Route("api")]
public class PingController : ControllerBase
{
    private readonly ILogger<PingController> _logger;

    public PingController(ILogger<PingController> logger)
    {
        _logger = logger;
    }

    [HttpGet("ping/{word?}")]
    public string Ping(string word = "pong")
    {
        return word;
    }
}
