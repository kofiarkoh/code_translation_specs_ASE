            #include <stdio.h>
            int main(){
             
            int a,b,c;
            scanf("%d%d%d",&a,&b,&c);
             
            int min;
            min=a+b; 
            if(min>a+c) min=a+c;
            else 
            if(min>b+c) min=b+c;
            else min=min;
            printf("%d",min);
             
             
            return 0;  
            }