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

    // TODO: make this a hook
    public interface Response { }
    public record Success(object data) : Response;
    public record Error(Exception error) : Response;

    [HttpGet("ping/{word?}")]
    public Response Ping(string word)
    {
        if (string.IsNullOrEmpty(word))
        {
            return new Success("pong");
        }

        return new Success(new string(word.Reverse().ToArray()));
    }
}
