    #include<stdio.h>
     
    int main(){
      int N, K, i, j, t, sum;
      int l[51];
      
      scanf("%d %d",&N,&K);
     
      for(i = 0; i < N; i++){
        scanf("%d", &l[i]);
      }
     
      for(i = 0; i < K; i++){
        for(j = 0; j < N; j++){
          if(l[t] < l[j]){
    	
    	t = j;
          
          }
        }
        
        sum += l[t];
        l[t] = 0;
      
      }
      
      printf("%d",sum);
      return 0;
    }