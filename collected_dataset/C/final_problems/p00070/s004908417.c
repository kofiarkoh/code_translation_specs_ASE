#include <stdio.h>
int main(void){
    int n,s,x,a[10],e[11],c,i,j,m;
    while(~scanf("%d%d",&n,&s)){
        x=c=m=0;
        for(i=0;i<11;i++)e[i]=0;
        for(i=0;i<n;i++){
            a[i]=n-i-1;
            e[a[i]]=1;
            x+=a[i]*(i+1);
        }
        if(s>330) m=n;
        do{
            if(x==s)c++;
            if(x>=s)m++;
            for(i=0;i<n;i++){
                x-=a[i]*(i+1);
                e[a[i]]=0;
                if(i<m)continue;
                if(i==0)
                    a[i]=(s-x<10&&!e[s-x]?s-x:10);
                else
                    do{a[i]++;}while(e[a[i]]);
                if(a[i]>9){m++;continue;}
                e[a[i]]=1;
                x+=a[i]*(i+1);
                for(i--;i>=0;i--){
                    for(j=0;j<10;j++){
                        if(e[j])continue;
                        a[i]=j;
                        e[a[i]]=1;
                        x+=a[i]*(i+1);
                        break;
                    }
                }
                if(x>s) {
                    m++;
                    if(m>=n) break;
                    continue;
                }
                m=0;
                break ;
            }
        } while(m<n);
        printf("%d\n",c);
    }
    return 0;
}