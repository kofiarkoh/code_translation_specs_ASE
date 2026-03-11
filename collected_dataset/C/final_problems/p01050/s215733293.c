#include <stdio.h>
#include <string.h>
int main(void){
        char str[101];
        int alp[26] = {},num[10] = {};
        char *s = str;
        int *a = alp, *n = num;
        int i,j,f,c = 0;
        scanf("%s",str);

        while(*s){
                if(*s >= 'a' && *s <= 'z')
                        *(a + (*s-'a')) += 1;
                else if(*s >= '0' && *s <= '9')
                        *(n + (*s-'0')) += 1;
                s++;
        }
for(j=0;j<100;j++){
        f = 0;
        for(i = 0; i<26; i++){
                if(*(a+i) > 0){
                        f++;
                }else{
                        if(f > 3)
                                c += f-3;
                        f = 0;
                }
                *(a+i) -= 1;
        }
        if(f>3)
                c += f-3;

        f = 0;
        for(i = 0; i<10; i++){
                if(*(n+i) > 0){
                        f++;
                }else{
                        if(f > 3)
                                c += f-3;
                        f = 0;
                }
                *(n+i) -= 1;
        }
        if(f>3)
                c += f-3;
}
        printf("%d\n",strlen(str)-c);

        return 0;
}
