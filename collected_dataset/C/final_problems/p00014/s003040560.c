#include <stdio.h>
int main(void){
    int d;
    long area=0;
    int i;
    
    while(scanf("%d",&d) != EOF){
        area=0;
        for(i=d; i<=600-d; i+=d){
            area+=i*i*d;
        }
        printf("%ld\n",area);
    }
    
    return 0;
}