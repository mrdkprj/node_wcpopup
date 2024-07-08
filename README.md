# win32props
Reads and writes Windows file property values.  

## Functions

### read(file:string, format?:boolean) => Promise\<object\>

Gets data for all available properties of a file. If format is true, values are formatted for display.  
[Windows Properties](https://learn.microsoft.com/en-us/windows/win32/properties/props)  

### getValue(file:string, propertyName:string) => Promise\<string\>
Gets the data for the specified property of a file.  

### getValues(files:string, propertyName:string) => Promise\<object\>
Gets the data for the specified property of files.  
All files must be in the same folder.  

### setValue(file:string, propertyName:string, propertyValue:string) => Promise\<boolean\>
Sets the data for the specified property of a file.  
This function requires lock on the file. If the file is used by another process, this operation fails.  