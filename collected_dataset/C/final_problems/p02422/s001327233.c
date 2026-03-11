#include<stdio.h>
#include<string.h>
  
int main(void){
    char str[1001];     // input; input string , 1<=length<=1000
    int n=0;            // input; command times, 1<=n<=100
    int a=0, b=0;
    char* p_str=str;    // string pointer
    char cpy[1001];
    int i=0, j=0;       // loop index
  
    scanf("%s", str);
    scanf("%d", &n);
    for(i=0;i<n;i++){
        scanf("%s", cpy);
        if(strcmp(cpy, "print") == 0){
            scanf("%d %d", &a, &b);
            p_str = str + a;
            strncpy(cpy, p_str, b-a+1);
            cpy[b-a+1] = '\0';
            printf("%s\n", cpy);
  
        }else if(strcmp(cpy, "reverse") == 0){
            scanf("%d %d", &a, &b);
            p_str = str + b;
            for(j=0;j<b-a+1;j++){
                cpy[j] = *(p_str);
                p_str--;
            }
            p_str = str + a;        // ??????¶???????????????
            for(j=0;j<b-a+1;j++) *(p_str + j) = cpy[j];
  
        }else if(strcmp(cpy, "replace") == 0){
            scanf("%d %d %s", &a, &b, cpy);
            p_str = str + a;
            for(j=0;j<b-a+1;j++) *(p_str + j) = cpy[j];
          
        }else{
          
        }
    }
    return 0;
}
