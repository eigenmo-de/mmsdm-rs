# aemo-rs

## To get the table_config, 
0. Go to: https://di-help.docs.public.aemo.com.au/Content/Data_Interchange/SoftwareReleases.htm?TocPath=_____17
1. Downlaod `SQL Server DI bundle`
2. Unzip pdrLoaderConfiguration/*.zip
3. Remove everything except I & D rows for `TABLE`
4. Remove the config columns (first four)
5. Name as `table_config_v{CURRENT_VERSION}.csv`