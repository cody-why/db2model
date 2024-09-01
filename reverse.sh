sql_reverse mysql -f mytera/reverse.yml -p 'mytera/*' -n rbatis.tera -c mytera/mysql_rbatis.json
sql_reverse mysql -f mytera/reverse_handler.yml -p 'mytera/*' -n handler.tera -c mytera/mysql_rbatis.json
sql_reverse mysql -f mytera/reverse_service.yml -p 'mytera/*' -n service.tera -c mytera/mysql_rbatis.json
sql_reverse mysql -f mytera/reverse_vo.yml -p 'mytera/*' -n vo.tera -c mytera/mysql_rbatis.json
