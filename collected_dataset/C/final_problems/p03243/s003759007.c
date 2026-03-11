#include <stdio.h>
int main(void){
    // Your code here!
    int N;
    scanf("%d",&N);
    
    if(N%111!=0) N = N - N%111 + 111;
    
    printf("%d\n",N);
    
    return 0;
    
}
