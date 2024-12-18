using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Logging;
using packrat.dataAccessLayer.Context;

namespace packrat.dataAccessLayer.Services;

public interface IUserDbService
{
    public Task<User?> GetUserById(int id);
    public Task<User?> GetUserByEmail(string email);
    public Task<User> CreateUser(string email, string password);
}

public class UserDbService : IUserDbService
{
    private readonly ILogger<UserDbService> _logger;
    private readonly PackratContext _context;

    public UserDbService(ILogger<UserDbService> logger, PackratContext context)
    {
        _logger = logger;
        _context = context;
    }

    public async Task<User?> GetUserById(int id)
    {
        return await _context.Users.FindAsync(id);
    }

    public async Task<User?> GetUserByEmail(string email)
    {
        return await _context.Users.FirstOrDefaultAsync(x => x.Email == email);
    }

    public async Task<User> CreateUser(string email, string password)
    {
        var user = new User
        {
            Email = email,
            Password = password
        };
        var newUser = await _context.Users.AddAsync(user);
        await _context.SaveChangesAsync();
        return newUser.Entity;
    }
}