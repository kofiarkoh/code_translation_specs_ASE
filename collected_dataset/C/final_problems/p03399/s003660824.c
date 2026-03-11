#include <stdio.h>

int a,b,c,d,ans ;

int main(void){
    
    scanf ("%d%d%d%d",&a,&b,&c,&d) ;
    
    if ( a<=b ){
        ans += a ;
    } else {
        ans += b ;
    }
    
    if ( c<=d ){
        ans += c ;
    } else {
        ans += d ;
    }
    
    printf ("%d\n",ans) ;
    
    return 0 ;
}