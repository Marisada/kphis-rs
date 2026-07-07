@echo off

set pwa_path=volume\pwa
set css_path=statics\css

echo create %pwa_path%\app.min.css
grass -s compressed sass\src\app.scss %pwa_path%\app.min.css

echo create %pwa_path%\%css_path%\bootstrap.min.css
grass -s compressed sass\src\bootstrap\scss\bootstrap.scss %pwa_path%\%css_path%\bootstrap.min.css

echo create %pwa_path%\%css_path%\font-awesome.min.css
grass -s compressed sass\src\font-awesome\scss\fontawesome.scss %pwa_path%\%css_path%\font-awesome.min.css

echo create %pwa_path%\%css_path%\font-awesome-solid.min.css
grass -s compressed sass\src\font-awesome\scss\solid.scss %pwa_path%\%css_path%\font-awesome-solid.min.css

echo create %pwa_path%\%css_path%\font-awesome-regular.min.css
grass -s compressed sass\src\font-awesome\scss\regular.scss %pwa_path%\%css_path%\font-awesome-regular.min.css

echo precompress %pwa_path%\app.min.css
precompress -c br:11,gz:9 %pwa_path%\app.min.css

echo precompress %pwa_path%\%css_path%\bootstrap.min.css
precompress -c br:11,gz:9 %pwa_path%\%css_path%\bootstrap.min.css

echo precompress %pwa_path%\%css_path%\font-awesome.min.css
precompress -c br:11,gz:9 %pwa_path%\%css_path%\font-awesome.min.css

echo:
reload