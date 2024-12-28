using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.Logging;
using packrat.dataAccessLayer.Services;

namespace packrat.Services.Services.Authentication;

public interface IAuthenticationService
{
    public Task<bool> Authenticate(string email, string password);
}

public class AuthenticationService : IAuthenticationService
{
   private readonly ILogger<AuthenticationService> _logger;
   private readonly IUserDbService _userService;

   public AuthenticationService(ILogger<AuthenticationService> logger, IUserDbService userService)
   {
       _logger = logger;
       _userService = userService;
   }

   public async Task<bool> Authenticate(string email, string password)
   {
       var user = await _userService.GetUserByEmail(email);
       if (user == null) return false;

       return BCrypt.Net.BCrypt.EnhancedVerify(password, user.Password);
   }
}