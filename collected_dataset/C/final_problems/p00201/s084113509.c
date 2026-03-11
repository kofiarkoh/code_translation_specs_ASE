#include<stdio.h>
#include<stdlib.h>
#include<string.h>

int n,m;
struct element{
  char name[101];
  int cost;
} *product;

struct recipe{
  char name[101];
  int num_of_prod;
  char zairyo[100][101];
} *menu;

void minimum_cost(int number);

int main(void){
  int i,j;
  char final[101];
  while(scanf("%d",&n) && n){
    product=(struct element *)calloc(n,sizeof(struct element));
    for(i=0;i<n;i++){
      scanf("%s %d%*c", product[i].name,&product[i].cost);
    }
    /*
    for(i=0;i<n;i++){
      printf("%s %d\n", product[i].name,product[i].cost);
      }*/

    scanf("%d",&m);
    menu=(struct recipe *)calloc(m,sizeof(struct recipe));
    for(i=0;i<m;i++){
      scanf("%s %d%*c", menu[i].name,&menu[i].num_of_prod);
      for(j=0;j<menu[i].num_of_prod;j++){
	scanf("%s%*c",menu[i].zairyo[j]);
      }
    }
    /*
    for(i=0;i<m;i++){
      printf("%s %d ", menu[i].name,menu[i].num_of_prod);
      for(j=0;j<menu[i].num_of_prod;j++){
	printf("%s ",menu[i].zairyo[j]);
      }
      putchar('\n');
      }*/
    
    scanf("%s%*c",final);
    for(i=0;i<n;i++){
      if(!strcmp(final,product[i].name)){
	minimum_cost(i);
	printf("%d\n",product[i].cost);
	break;
      }
    }
    
    free(menu);
    free(product);
  }
  return 0;
}

void minimum_cost(int number){
  int i,j,k;
  int sum=0;
  //printf("%d %d\n",m,n);
  //printf("%s recipe is searching...\n",product[number].name);
  for(i=0;i<m;i++){
    //printf("%lu %lu\n",strlen(product[number].name),strlen(menu[i].name));
    if(!strcmp(product[number].name,menu[i].name)){
      //printf("%s menu found!\n",product[number].name);
      for(j=0;j<menu[i].num_of_prod;j++){
	for(k=0;k<n;k++){
	  if(!strcmp(menu[i].zairyo[j],product[k].name)){
	    minimum_cost(k);
	    sum+=product[k].cost;
	    break;
	  }
	}	
      }
      //printf("%s cost is from %d to ",product[number].name,product[number].cost);
      if(sum<product[number].cost) product[number].cost=sum;
      //printf("%d \n",product[number].cost);
      break;
    }
  }
}