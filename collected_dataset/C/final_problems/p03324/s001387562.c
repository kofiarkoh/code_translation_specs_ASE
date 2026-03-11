#include<stdio.h>


int main(){
    int d , n ;
    int i;

    scanf("%d %d",&d,&n);

    if( d == 0 && n != 100){
        printf("%d\n",n);
    }
    else if ( d == 1 && n != 100 ){
        printf("%d\n", 100 * n );
    } 
    else if ( d ==2  && n != 100 ){
        printf("%d\n", 100 * 100 * n );
    }       
    if( d == 0 && n == 100){
        printf("%d\n",101);
    }
    else if ( d == 1 && n == 100 ){
        printf("%d\n", 10001 );
    } 
    else if ( d == 2 && n == 100 ){
        printf("%d\n", 1000001 );
    }        


    return 0 ;

}