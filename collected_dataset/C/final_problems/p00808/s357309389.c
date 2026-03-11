#define INF 10000000
#define BIAS 24*60
#include <stdlib.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

char gCity[2001][64];
int gCityNum = 0;
int from_hakodate[2001][101];
int to_hakodate[2001][101];
int from_tokyo[2001][101];
int to_tokyo[2001][101];

struct Train{
  int from, to;
  int dpt, arv;
  int fare;
};

int max(int lhs,int rhs){
  if(lhs > rhs) return lhs;
  else return rhs;
}

int min(int lhs,int rhs){
  if(lhs > rhs) return rhs;
  else return lhs;
}

int getCityNum(char* cityName){
  int i;
  for(i=0;i<gCityNum;i++){
    if(strcmp(gCity[i],cityName)==0) return i;
  }
  
  strcpy(gCity[gCityNum],cityName);
  return gCityNum++;
}

void setTrain(int idx,struct Train* trains,
	      int dptH,int dptM,int arvH,int arvM,int fare,
	      char dptCity[64],char arvCity[64]){
  trains[idx].from = getCityNum(dptCity);
  trains[idx].to = getCityNum(arvCity);
  trains[idx].dpt = dptH*60 + dptM;
  trains[idx].arv = arvH*60 + arvM;
  trains[idx].fare = fare;
}

void setRTrain(int idx,struct Train* trains,
	      int dptH,int dptM,int arvH,int arvM,int fare,
	      char dptCity[64],char arvCity[64]){
  trains[idx].from = getCityNum(arvCity);
  trains[idx].to = getCityNum(dptCity);
  trains[idx].dpt = BIAS - (arvH*60 + arvM);
  trains[idx].arv = BIAS - (dptH*60 + dptM);
  trains[idx].fare = fare;
}

int seekMaxFareTrain(struct Train* trains,int ub){
  int i;
  int maxFare = -1;
  int res;
  for(i=0;i<ub;i++){
    if(maxFare < trains[i].fare){
      maxFare = trains[i].fare;
      res = i;
    }
  }
  return res;
}

int seekMostDelayTrain(struct Train* trains,int ub){
  int i;
  int mostDelay = -1;
  int res;
  for(i=0;i<ub;i++){
    if(mostDelay < abs(trains[i].dpt - trains[i].arv)){
      mostDelay = abs(trains[i].dpt - trains[i].arv);
      res = i;
    }
  }
  return res;
}

int change(int city,int id,struct Train* trains){
  int i;
  for(i=id-1;i>=0;i--){
    if(trains[i].arv <= trains[id].dpt
       && trains[i].to == city){
      return i;
    }
  } 

  return 0;
}

int fr(char* name,int trainNum,int table[2001][101], struct Train* trains){
  int src = getCityNum(name);

  //printf("%s %d\n",name,src);

  table[0][src] = 0;
  int i;
  for(i=0;i<trainNum;i++){
    int j;
    for(j=0;j<gCityNum;j++){
      if(j==trains[i].to){
	table[i+1][j] = min(table[i][j],table[change(trains[i].from,i,trains)+1][trains[i].from]+trains[i].fare);
      }
      else table[i+1][j] = table[i][j];
    }
  }
}

void printTable(int table[2001][101],int n){
  int i;
  for(i=0;i<=n;i++){
    int j;
    for(j=0;j<gCityNum;j++){
      printf("%d ",table[i][j]);
    }
    printf("\n");
  }
}

int mycmp(const void* t1,const void* t2){
  return ((const struct Train*)t1)->arv - ((const struct Train*)t2)->arv;
}

void printTrains(struct Train* trains,int n){
  int i;
  for(i=0;i<n;i++){
    printf("%d %d %d %d %d\n",
	   trains[i].from,trains[i].dpt,trains[i].to,
	   trains[i].arv,trains[i].fare);
  }
}

int calc_cost(int city,struct Train* trains,struct Train* rtrains,int trainNum){
  int i;
  int res = INF;
  for(i=0;i<trainNum;i++){
    int j;
    for(j=trainNum-1;j>=0;j--){
      int stay = (BIAS - rtrains[j].arv) - trains[i].arv;
      if(stay < 30) continue;

      int c = from_hakodate[i+1][city]
	+ from_tokyo[i+1][city]
	+ to_hakodate[j+1][city]
	+ to_tokyo[j+1][city];
      res = min(res,c);
    }
  }
  return res;
}

int main(){
  int n;
  while(~scanf("%d",&n)){
    if(n==0) break;
    struct Train* trains = (struct Train*)malloc(n*sizeof(struct Train));
    struct Train* rtrains = (struct Train*)malloc(n*sizeof(struct Train));
    int i=0;
    int j=0;
    for(i=0,j=0;i<n;i++){
      int dptH,dptM,arvH,arvM,fare;
      char dptCity[64],arvCity[64];
      scanf("%s %d:%d %s %d:%d %d",dptCity,&dptH,&dptM,arvCity,&arvH,&arvM,&fare);
      if((dptH*60 + dptM < 8*60) || (arvH*60 + arvM > 18*60)) continue;
      setTrain(j,trains,dptH,dptM,arvH,arvM,fare,dptCity,arvCity);
      setRTrain(j++,rtrains,dptH,dptM,arvH,arvM,fare,dptCity,arvCity);
    }

    //    printTrains(trains,j);
    //    printf("\n");
    qsort(trains,j,sizeof(struct Train),mycmp);
    qsort(rtrains,j,sizeof(struct Train),mycmp);

    //    printTrains(trains,j);
    //    printf("\n");

    memset(from_hakodate,0x3,sizeof(from_hakodate));      
    fr("Hakodate",j,from_hakodate,trains);

    memset(from_tokyo,0x3,sizeof(from_tokyo));      
    fr("Tokyo",j,from_tokyo,trains);

    memset(to_hakodate,0x3,sizeof(to_hakodate));      
    fr("Hakodate",j,to_hakodate,rtrains);

    memset(to_tokyo,0x3,sizeof(to_tokyo));      
    fr("Tokyo",j,to_tokyo,rtrains);


    int mincost = INF;
    
    for(i=0;i<gCityNum;i++){
      mincost = min(calc_cost(i,trains,rtrains,j),mincost);
    }
    //printTable(to_hakodate,j);
    //printTable(to_tokyo,j);
    
    //printf("\n");
    //printf("cityname:%s cityid:%d\n",arvCity,getCityNum(arvCity));

    printf("%d\n",mincost == INF ? 0:mincost);

    memset(gCity,'\0',sizeof(gCity));
    gCityNum = 0;
    free(trains);
    free(rtrains);
  }
  return 0;
}