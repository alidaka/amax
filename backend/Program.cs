var builder = WebApplication.CreateBuilder(args);

builder.Services.AddControllers();

var app = builder.Build();

// TODO: might only want this if `!app.Environment.IsDevelopment()`
app.UseStaticFiles();

app.MapControllers();
app.UseHttpsRedirection();
app.UseAuthorization();

app.Run();