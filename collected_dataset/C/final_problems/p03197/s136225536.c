#include <stdio.h>
typedef unsigned int uint;
inline static uint gl(void){
    uint x = 0;
    for(;;){
        const uint c = getchar_unlocked() - 48;
        if(c < 10){
            x = x*10 + c;
        }else{
            break;
        }
    }
    return x;
}

int main(void){
    long n = gl();
    for(long i=0; i<n; i++){
        long t = gl();
        if(t%2==1){
            puts("first");
            return 0;
        }
    }
    puts("second");
    return 0;
}
