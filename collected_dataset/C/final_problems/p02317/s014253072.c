#include<stdio.h>

int main(){
    int n;
    scanf("%d",&n);
    int a[n];
    int aidx[n];
    int increase[n+1];
    int incmax=1;
    int i,j;
    
    for(i=0;i<n+1;i++){
        increase[i]=1000000001;
        if(i<n){
            a[i]=0;
            aidx[i]=0;
        }
    }
    
    for(i=0;i<n;i++){
        scanf("%d",&a[i]);
        //printf("a[%d]:%d\n",i,a[i]);
        if(i==0){
            increase[1]=a[i];
            aidx[0]=1;
        }else{
            int t_aidx;
            t_aidx=0;
            for(j=i-1;j>-2;j--){
                if(j==-1){
                    if(t_aidx==0){
                        if(increase[1]>a[i]){
                            increase[1]=a[i];
                        }
                        aidx[i]=1;
                    }
                }else if(a[j]<a[i]){
                    if(aidx[j]>t_aidx){
                        t_aidx=aidx[j];
                        aidx[i]=t_aidx+1;
                    }
                }
            }
            if(increase[aidx[i]]>a[i]){
                increase[aidx[i]]=a[i];
            }
            if(aidx[i]>incmax){
                incmax=aidx[i];
            }
        }
        /*printf("increase:");
        for(j=0;j<n+1;j++){
            printf("%d,",increase[j]);
        }
        printf("\naidx:");
        for(j=0;j<n;j++){
            printf("%d,",aidx[j]);
        }
        printf("\n");*/
    }
    /*printf("   a:");
    for(j=0;j<n;j++){
        printf("%d,",a[j]);
    }
    printf("\n");*/
    printf("%d\n",incmax);
    return 0;
}