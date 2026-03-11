#include <stdio.h>

int main(void){
    //long long int num[6] = {};
    long long int ninzu,temp;
    long long int trans[5];
    long long int min = 0;
    int i,bneck;
    
    scanf("%lld", &ninzu);
    for(i = 0; i < 5; i++){
        scanf("%lld", &trans[i]);
    }
    
    
    temp = 10e14;
    for(i = 0; i < 5; i++){
        if(temp >= trans[i]){
            bneck = i;
            temp = trans[i];
        }
    }
    
    //printf("bneck:%d\n", bneck);
    
    min += (ninzu + trans[bneck] - 1)/trans[bneck];
    min += 4;
    
    printf("%lld\n", min);
    
    return 0;
}