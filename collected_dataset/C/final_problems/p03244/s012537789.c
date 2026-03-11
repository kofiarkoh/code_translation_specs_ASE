
#include <stdio.h>
int MIN(int a,int b){
    return a<b?a:b;
}
int MAX(int a,int b){
    return a>b?a:b;
}
int main()
{
    int n;
    scanf("%d",&n);
    int v[n],odd[n/2+1],even[n/2+1];
    int j=0,k=0;
    for(int i=0;i<n;i++){
        scanf("%d",&v[i]);
        if(i%2==0){
            odd[j]=v[i];
            j++;
        }else{
            even[k]=v[i];
            k++;
        }
    }
    int no[100001]={0},ne[100001]={0};
    for(int i=0;i<k;i++){
        ne[even[i]]++;
    }
    int po1,po2,pe1,pe2,pno1=0,pno2,pne1=0,pne2;
    for(int i=0;i<j;i++){
        no[odd[i]]++;
    }
    for(int i=0;i<100001;i++){
        if(pno1<=no[i]){
            po2=po1;
            pno2=pno1;
            po1=i;
            pno1=no[i];
        }
        if(pne1<=ne[i]){
            pe2=pe1;
            pne2=pne1;
            pe1=i;
            pne1=ne[i];
        }
        
    }
    long long ans=0;
    if(pe1==po1){
        ans=MAX(pno1+pne2,pno2+pne1);
    }else{
        ans=pno1+pne1;
    }
    printf("%lld",n-ans);
    
    
    
    

    return 0;
}
