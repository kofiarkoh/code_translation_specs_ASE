#include <stdio.h>
#include <string.h>
 
int main(){
                int i, N, len, cnt, tmp;
                char str[1001];
                 
                scanf("%d", &N);
                for(i=0;i<N;i++){
                                scanf("%s", str);
                                cnt = 0;
                                len = strlen(str) - 1;
                                while(len > 1){
                                                tmp = (str[len] - '0') + (str[len - 1] - '0');
                                                if(tmp > 9){
                                                                str[len-1] = '1';
                                                                str[len] = (char)((tmp % 10) + '0');
                                                                str[len+1] = '\0';
                                                }else{
                                                                str[len - 1] = (char)(tmp + '0');
                                                                str[len] = '\0';
                                                                len--;
                                                }
                                                cnt++;
                                }
                                printf("%s\n", (cnt % 2) ? "Fabre wins." : "Audrey wins.");
                }
                return 0;
}