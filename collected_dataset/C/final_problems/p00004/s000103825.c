#include <stdio.h>

int main( void )
{
    int a,b,c,d,e,f;
    float x,y;
    
    while(scanf("%d %d %d %d %d %d",&a,&b,&c,&d,&e,&f)!=-1){    
        y = (float)( c*d -a*f ) / (b*d-a*e); 
        x = (float)( c*e -b*f ) / (a*e-b*d);
        if(-0.0005<x && x<=0 ){
            x = 0;
        }
        if(-0.0005<y && y<=0 ){
            y = 0;
        }
        printf( "%.3lf %.3f\n",x,y); 
    }
    return 0;
}