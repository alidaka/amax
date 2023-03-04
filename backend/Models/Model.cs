using Microsoft.EntityFrameworkCore;
using System;
using System.Collections.Generic;

public class AmaxContext : DbContext
{
    public DbSet<User> Users { get; set; }
    public DbSet<Post> Posts { get; set; }
    public DbSet<Comment> Comments { get; set; }

    // TODO: parameterize for deployment
    protected override void OnConfiguring(DbContextOptionsBuilder options)
        => options.UseNpgsql("Host=localhost;Database=amax-dev");
}

public class User
{
    public int Id { get; set; }
    public string Name { get; set; }

    public List<Post> Posts { get; } = new();
    public List<Comment> Comments { get; } = new();
}

public class Post
{
    public int Id { get; set; }
    public string Title { get; set; }
    public string Content { get; set; }
    public DateTime Posted { get; set; }

    public int PoserId { get; }
    public User Poster { get; }

    public List<Comment> Comments { get; } = new();
}

public class Comment
{
    public int Id { get; set; }
    public string Content { get; set; }

    public int PosterId { get; set; }
    public User Poster { get; set; }
    public int PostId { get; set; }
    public Post Post { get; set; }
    public int? ParentId { get; set; }
    public Comment? Parent { get; set; }
}