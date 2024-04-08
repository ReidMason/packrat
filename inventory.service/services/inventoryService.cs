using inventory.client.dtos;
using Microsoft.Extensions.Logging;
using inventory.data.Contexts;
using inventory.data.Models;

namespace inventory.service.services;

public interface IInventoryService {
  public List<LocationDto> GetAllLocations();
  public void CreateLocation(NewLocationDto newLocation);
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

    public void CreateLocation(NewLocationDto newLocation)
    {
      _dbContext.Locations.Add(new Location { Name = newLocation.Name });
      _dbContext.SaveChanges();
    }

    public List<LocationDto> GetAllLocations()
  {
    return _dbContext.Locations.Select(x => new LocationDto {
      Id = x.Id,
      Name = x.Name
    }).ToList();
  }
}
