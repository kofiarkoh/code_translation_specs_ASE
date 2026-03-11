#include<stdio.h>

int main(void)
{
    int i;
    int n[25]={0},tem[6]={24,24,24,24,24,24};
    int max1=24,max2=24;
    double s[25]={0};



    for(i=0;i<24;i++){
        scanf("%d%lf",&n[i],&s[i]);
        if(i%8==0){
        s[24]=1000;
        }
        if(s[max1]>s[i]){
            tem[(i/8)*2+1]=tem[(i/8)*2];
            tem[(i/8)*2]=max2;
            max2=max1;
            max1=i;
        } else if(s[max2]>s[i]){
            tem[(i/8)*2+1]=tem[(i/8)*2];
            tem[(i/8)*2]=max2;
            max2=i;
        } else if(s[tem[(i/8)*2]]>s[i]){
            tem[(i/8)*2+1]=tem[(i/8)*2];
            tem[(i/8)*2]=i;
        } else if(s[tem[(i/8)*2+1]]>s[i]){
            tem[(i/8)*2+1]=i;
        }

        if(i%8==7){
            printf("%d %.2lf\n%d %.2lf\n",n[max1],s[max1],n[max2],s[max2]);
            max1=24;
            max2=24;
        }
    }

    for(i=0;i<6;i++){
        if(s[max1]>s[tem[i]]){
            max2=max1;
            max1=tem[i];
        } else if(s[max2]>s[tem[i]]){
            max2=tem[i];
        }
       // printf("%d ",tem[i]);
    }//printf("%d %d\n",max1,max2);

    printf("%d %.2lf\n%d %.2lf\n",n[max1],s[max1],n[max2],s[max2]);

    return 0;
}