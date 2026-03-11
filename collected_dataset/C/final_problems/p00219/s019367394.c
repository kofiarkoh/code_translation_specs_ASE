#include<stdio.h>
int main (void){
    int a,i,j,n;
    for(;;){
        scanf("%d",&n);
        if(n==0)break;
        int k[11]={0};
        for(i=0;i<n;i++){
            scanf("%d",&a);
            if(a==0)k[0]++;
            else if(a==1)k[1]++;
            else if(a==2)k[2]++;
            else if(a==3)k[3]++;
            else if(a==4)k[4]++;
            else if(a==5)k[5]++;
            else if(a==6)k[6]++;
            else if(a==7)k[7]++;
            else if(a==8)k[8]++;
            else if(a==9)k[9]++;
        }
        for(i=0;i<10;i++){
            if(k[i]==0)printf("-\n");
            for(j=0;j<k[i];j++){
                printf("*");
                if(j==k[i]-1)printf("\n");
            }
        }
    }
return 0;
}