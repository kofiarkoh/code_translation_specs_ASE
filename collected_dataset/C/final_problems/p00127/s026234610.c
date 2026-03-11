#include <stdio.h>

char map[6][6]={"abcde","fghij","klmno","pqrst","uvwxy","z.?! "};

int main(void){
    int n,i,sign,t;
    char line[201],ans[101];
    for(;i=0,t=0,sign=0,~scanf("%s",line);){
        while(line[i]){
            if(line[i]-48<1 || line[i]-48>6 || line[i+1]-48<1 || line[i+1]-48 >5){
                sign=1;break;
            }
            else if(line[i+1]==0){
                sign=1;break;
            }
            ans[t++]=map[line[i]-1-48][line[i+1]-1-48];
            i+=2;
        }
        ans[t]=0;
        if(sign)
            puts("NA");
        else
            puts(ans);
    }
    return 0;
}