#include <stdio.h>

int ans=700 ;
char s[3] ;

int main(void){
    
    scanf ("%s",s) ;
    
    if (s[0]=='o') {
        ans +=100 ;
    }
    
    if (s[1]=='o') {
        ans +=100 ;
    }
    
    if (s[2]=='o') {
        ans +=100 ;
    }
    
    printf ("%d",ans) ;

    return 0 ;
}
