#include <stdio.h>
#include <ctype.h>
 
unsigned int data[8][2] = {
    1826015432,2468951863,
    4148788223,146179072,
    2147300159,2147667136,
    1813301512,332083959,
    1577336967,555301496,
    1359814671,771756592,
    268509233,17306118,
    16846885,268968466,
};
 
unsigned int count(unsigned int n){
    n = (n & 0x55555555) + ((n & 0xAAAAAAAA) >> 1);
    n = (n & 0x33333333) + ((n & 0xCCCCCCCC) >> 2);
    n = (n & 0x0F0F0F0F) + ((n & 0xF0F0F0F0) >> 4);
    n = (n & 0x00FF00FF) + ((n & 0xFF00FF00) >> 8);
    n = (n & 0x0000FFFF) + ((n & 0xFFFF0000) >> 16);
     
    return n;
}
 
int main(void) {
    int tmp,num,i,j,p;
    unsigned check;
    char c,kigo[7] = " .,-'?",kigo2[7] = " ',-.?";
     
    p = 0;
    check = 0xFFFFFFFF;
     
    while((c = getchar()) != EOF){
        if(isalpha(c)){
            num = c-'A';
        }else if(c == '\n'){
            putchar('\n');
            p = 0;
            check = 0xFFFFFFFF;
            continue;
        }else{
            for(num = 0;num < 6;num++){
                if(c == kigo[num]){
                    num += 26;
                    break;
                }
            }
        }
        for(i = 4;i >= 0;i--){
            tmp = (num&(1<<i))>>i;
            check &= data[p++][tmp];
            if(count(check) == 1){
                for(j = 0;j < 32;j++){
                    if(check&(1<<(31-j))){
                    	putchar(j < 6 ? kigo2[j] : j-6+'A');
                        break;
                    }
                }
                check = 0xFFFFFFFF;
                p = 0;
            }
        }
    }
    
    return 0;
}