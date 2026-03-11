#include <stdio.h>
#include <string.h>
int main(){
    char arr[25];
    int i,n;
    gets(arr);
    n=strlen(arr);
    for(i=0;i<n;i++){
        if(arr[i]==',') arr[i]=' ';
    }
    printf("%s\n",arr);
    return 0;
}
