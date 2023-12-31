### 実践Rustプログラミング入門の勉強記録

Updateに伴い生じたエラーの修正などを残しておく

# §5-1
変更不要
# §5-2
変更不要
# §5-3、5-4
# 変更前
```rs
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(add_todo)
            .service(delete_todo)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(()) 
```
# 変更後
```rs
    // バージョン変更に伴いdata → app_dataに変更
    // data(pool.clone()) → .app_data(web::Data::new(pool.clone()))
    HttpServer::new(move || 
        App::new()
        .app_data(web::Data::new(pool.clone()))
        .service(index)
        )
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())    
