using DotNext;
using inventory.client.dtos;
using inventory.data.Contexts;
using inventory.data.Models;
using Microsoft.Extensions.Logging;

namespace inventory.service.services;

public interface IInventoryService
{
    public List<LocationDto> GetAllLocations();
    public Result<string, Errors> CreateLocation(NewLocationDto newLocation);
}

public enum Errors
{
    LocationAlreadyExists
}

public class InventoryService : IInventoryService
{
    private readonly ILogger<InventoryService> _logger;
    private readonly InventoryContext _dbContext;

    public InventoryService(ILogger<InventoryService> logger, InventoryContext dbContext)
    {
        _logger = logger;
        _dbContext = dbContext;
    }

    public Result<string, Errors> CreateLocation(NewLocationDto newLocation)
    {
        try
        {
            var existingLocation = _dbContext.Locations.FirstOrDefault(x =>
                x.Name == newLocation.Name
            );

            if (existingLocation != null)
                return new Result<string, Errors>(Errors.LocationAlreadyExists);

            _dbContext.Locations.Add(new Location { Name = newLocation.Name });
            _dbContext.SaveChanges();
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error creating location");
            return new Result<string, Errors>("Error creating location");
        }

        return "Location created successfully";
    }

    public List<LocationDto> GetAllLocations()
    {
        return _dbContext
            .Locations.Select(x => new LocationDto { Id = x.Id, Name = x.Name })
            .ToList();
    }
}
