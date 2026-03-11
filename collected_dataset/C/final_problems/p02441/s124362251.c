#include <stdio.h>

int counter(int *data,int b,int e,int k){
    int i;
    int count=0;
     for (i=b;i<e;++i){
     if (data[i]==k) count++;
     //printf("%d %d\n",data[0],data[4]);
     }
    return (count);
}

int main(){

    int i,n=1000,q=1000,ans[1000],e,b,k;
    int data[1000];
   
    for(i=0;i<5;++i){
	    data[i]=i; 
	}     
  
    //ans=counter(data,1,2,5);
    
    //printf ("ans= %3d",ans);


    scanf("%d",&n);
     
    
    for(i=0;i<n;++i){
	    scanf("%d",&data[i]); 
	}
	
    scanf("%d",&q);	
    
    for(i=0;i<q;++i){
	    scanf("%d",&b); 
	    scanf("%d",&e);
	    scanf("%d",&k);
	    ans[i]=counter(data,b,e,k);
	    //printf("ans= %d",ans);
	}	

    for(i=0;i<q;++i){
	    printf("%d\n",ans[i]); 
	}
    
    return 0;
}
