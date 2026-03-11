#include <stdio.h>
#include <string.h>
int main(void){
    int i,len,sum,num[100];
    char fig[100];
    
    while(scanf("%s",fig) != EOF){
        len=strlen(fig);
        
        for(i=0;i<len;i++){
            switch(fig[i]){
                case 'I' :num[i]=1;break;
                case 'V' :num[i]=5;break;
                case 'X' :num[i]=10;break;
                case 'L' :num[i]=50;break;
                case 'C' :num[i]=100;break;
                case 'D' :num[i]=500;break;
                case 'M' :num[i]=1000;break;
            }
        }
        for(i=0;i<len-1;i++){
            if(num[i]<num[i+1]){
                num[i+1]-=num[i];
                num[i]=0;
            }
        }
        sum=0;
        for(i=0;i<len;i++){
            sum+=num[i];
        }
        printf("%d\n",sum);
    }
    
    return 0;
}