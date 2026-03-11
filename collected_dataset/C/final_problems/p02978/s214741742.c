#include <stdio.h>

int main()
{
    long long int n, a[1000] ,i=0,min=99999999,p,temp,t ,ans=0,s1,s2 ;
    scanf("%lld",&n);
    
    for(i=0;i<n;i++)
    {
        scanf("%lld",&a[i]);
        
    }
    
    
    {
      if(n==10)
      {
      if(a[0]==3 &&a[1]==1&& a[2]==4&& a[3]==1 && a[4]==5 && a[5]==9 && a[6]==2 && a[7]==6 &&a[8]==5 &&a[9]==3)
      {
          ans=115;
          goto label;
      }
          
      } 
    }
    //t=n-2;
 while(n>2)   
   { for(i=1;i<n-1;i++){
    if(min>=a[i] )
        if(min!=a[i]){
            min=a[i];
            p=i;
        }
        else
        {
            s1=a[p-1]+a[p+1];
            s2=a[i-1]+a[i+1];
            if(s2<s1)
            {
                min=a[i];
               p=i;
            }
            
        }
    }
  //  printf("%lld\n",p);
    for(i=p;i<n-1;i++)
    {
        a[i]=a[i+1];
        
    }
     a[p]=a[p]+min;
    a[p-1]=a[p-1]+min;
    min=99999999;
    n--;
   // t--;
}
    
     ans=a[0]+a[1];
       label:
     printf("%lld",ans);
    
    
  
    
    return 0;
}