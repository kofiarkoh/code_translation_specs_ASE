#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DIJKSTRA_MAX_NODE 10000
#define DIJKSTRA_MAX_EDGE 40000

typedef struct {
	int from,to;
	int cost;
} dijkstra_edge_t;

typedef struct {
	int node;
	int cost;
} dijkstra_node_t;

int dijkstra_edge_num;
dijkstra_edge_t dijkstra_edge[DIJKSTRA_MAX_EDGE];
int dijkstra_node_edge[DIJKSTRA_MAX_NODE][2];
int dijkstra_queue_num;
dijkstra_node_t dijkstra_queue[DIJKSTRA_MAX_EDGE+1];
int dijkstra_cost[DIJKSTRA_MAX_NODE];
char dijkstra_visited[DIJKSTRA_MAX_NODE];

int dijkstra_comp(const void* x,const void* y) {
	const dijkstra_edge_t* a=(const dijkstra_edge_t*)x;
	const dijkstra_edge_t* b=(const dijkstra_edge_t*)y;
	if((a->from)>(b->from))return 1;
	if((a->from)<(b->from))return -1;
	if((a->to)>(b->to))return 1;
	if((a->to)<(b->to))return -1;
	return 0;
}

void dijkstra_queue_adjust(int pos) {
	int min=pos;
	if(pos*2+1<dijkstra_queue_num &&
		dijkstra_queue[min].cost>dijkstra_queue[pos*2+1].cost)
			min=pos*2+1;
	if(pos*2+2<dijkstra_queue_num &&
		dijkstra_queue[min].cost>dijkstra_queue[pos*2+2].cost)
			min=pos*2+2;
	if(min!=pos) {
		dijkstra_node_t temp;
		temp=dijkstra_queue[min];
		dijkstra_queue[min]=dijkstra_queue[pos];
		dijkstra_queue[pos]=temp;
		dijkstra_queue_adjust(min);
	} else if(pos>0)dijkstra_queue_adjust((pos-1)/2);
}

void dijkstra_queue_push(int node,int cost) {
	dijkstra_node_t topush;
	topush.node=node;topush.cost=cost;
	dijkstra_queue[dijkstra_queue_num++]=topush;
	dijkstra_queue_adjust(dijkstra_queue_num-1);
}

dijkstra_node_t dijkstra_queue_pop(void) {
	dijkstra_node_t result=dijkstra_queue[0];
	if(dijkstra_queue_num>0) {
		dijkstra_queue[0]=dijkstra_queue[--dijkstra_queue_num];
		dijkstra_queue_adjust(0);
	}
	return result;
}

void dijkstra_init(void) {
	dijkstra_edge_num=0;
}

void dijkstra_addedge(int from,int to,int cost) {
	dijkstra_edge[dijkstra_edge_num].from=from;
	dijkstra_edge[dijkstra_edge_num].to=to;
	dijkstra_edge[dijkstra_edge_num].cost=cost;
	dijkstra_edge_num++;
}

void dijkstra_setup(void) {
	int i;
	int prev;
	qsort(dijkstra_edge,dijkstra_edge_num,
		sizeof(dijkstra_edge_t),dijkstra_comp);
	prev=0;
	memset(dijkstra_node_edge,0,sizeof(dijkstra_node_edge));
	dijkstra_node_edge[0][0]=0;
	for(i=0;i<dijkstra_edge_num;i++) {
		if(dijkstra_edge[i].from!=dijkstra_edge[prev].from) {
			dijkstra_node_edge[dijkstra_edge[prev].from][1]=i;
			dijkstra_node_edge[dijkstra_edge[i].from][0]=i;
			prev=i;
		}
	}
	dijkstra_node_edge[dijkstra_edge[prev].from][1]=i;
}

int dijkstra_search(int from,int to) {
	int i;
	memset(dijkstra_cost,0xFF,sizeof(dijkstra_cost));
	memset(dijkstra_visited,0,sizeof(dijkstra_visited));
	dijkstra_queue_num=0;
	dijkstra_queue_push(from,0);
	while(dijkstra_queue_num>0) {
		dijkstra_node_t nownode=dijkstra_queue_pop();
		if(!dijkstra_visited[nownode.node]) {
			dijkstra_visited[nownode.node]=1;
			dijkstra_cost[nownode.node]=nownode.cost;
			for(i=dijkstra_node_edge[nownode.node][0];
					i<dijkstra_node_edge[nownode.node][1];i++) {
				if(!dijkstra_visited[dijkstra_edge[i].to] &&
						(dijkstra_cost[dijkstra_edge[i].to]<0 ||
						dijkstra_cost[dijkstra_edge[i].to]>nownode.cost+dijkstra_edge[i].cost)) {
					dijkstra_queue_push(
						dijkstra_edge[i].to,
						nownode.cost+dijkstra_edge[i].cost
					);
					dijkstra_cost[dijkstra_edge[i].to]=nownode.cost+dijkstra_edge[i].cost;
				}
			}
		}
	}
	return dijkstra_cost[to];
}

int zyunretu_num;
int zyunretu[120][5];

void make_zyunretu(int now,int max) {
	static int keiro[5];
	static int used[5];
	int i;
	if(now==0) {
		zyunretu_num=0;
		for(i=0;i<5;i++)keiro[i]=used[i]=0;
	}
	if(now<max) {
		for(i=0;i<max;i++) {
			if(!used[i]) {
				used[i]=1;
				keiro[now]=i;
				make_zyunretu(now+1,max);
				used[i]=0;
			}
		}
	} else {
		for(i=0;i<max;i++) {
			zyunretu[zyunretu_num][i]=keiro[i];
		}
		zyunretu_num++;
	}
}

int width,height;
char map[200][200];
char speed[200][200];
int sx,sy,dx,dy;
int m_num;
int mx[5],my[5];

int getnodenum(int x,int y) {
	return y*width+x;
}

int main(void) {
	int i,j;
	int result;
	scanf("%d%d",&width,&height);
	for(i=0;i<height;i++)scanf("%s",map[i]);
	m_num=0;
	for(i=0;i<height;i++) {
		for(j=0;j<width;j++)speed[i][j]=1;
	}
	for(i=0;i<height;i++) {
		for(j=0;j<width;j++) {
			if(map[i][j]=='S') {
				sy=i;sx=j;
			} else if(map[i][j]=='G') {
				dy=i;dx=j;
			} else if(map[i][j]=='M') {
				my[m_num]=i;mx[m_num]=j;
				m_num++;
			} else if(map[i][j]=='#') {
				int k,l;
				for(k=-2;k<=2;k++) {
					for(l=-2;l<=2;l++) {
						int kk,ll,s;
						kk=(k<0?-k:k);ll=(l<0?-l:l);
						if(kk<ll)kk=ll;
						s=4-kk;
						if(i+k>=0 && i+k<height && j+l>=0 && j+l<width &&
							speed[i+k][j+l]<s)speed[i+k][j+l]=s;
					}
				}
			}
		}
	}
	dijkstra_init();
	for(i=0;i<height;i++) {
		for(j=0;j<width;j++) {
			if(map[i][j]=='#')continue;
			if(i>0 && map[i-1][j]!='#') {
				dijkstra_addedge(
					getnodenum(j,i),getnodenum(j,i-1),speed[i][j]
				);
			}
			if(j>0 && map[i][j-1]!='#') {
				dijkstra_addedge(
					getnodenum(j,i),getnodenum(j-1,i),speed[i][j]
				);
			}
			if(i+1<height && map[i+1][j]!='#') {
				dijkstra_addedge(
					getnodenum(j,i),getnodenum(j,i+1),speed[i][j]
				);
			}
			if(j+1<width && map[i][j+1]!='#') {
				dijkstra_addedge(
					getnodenum(j,i),getnodenum(j+1,i),speed[i][j]
				);
			}
		}
	}
	dijkstra_setup();
	make_zyunretu(0,m_num);
	result=0x7fffffff;
	for(i=0;i<zyunretu_num;i++) {
		int nowresult=0;
		nowresult+=dijkstra_search(
			getnodenum(sx,sy),getnodenum(
				mx[zyunretu[i][0]],my[zyunretu[i][0]]));
		for(j=1;j<m_num;j++) {
			nowresult+=dijkstra_search(
				getnodenum(mx[zyunretu[i][j-1]],my[zyunretu[i][j-1]]),
				getnodenum(mx[zyunretu[i][j]],my[zyunretu[i][j]]));
		}
		nowresult+=dijkstra_search(
			getnodenum(mx[zyunretu[i][m_num-1]],my[zyunretu[i][m_num-1]]),
			getnodenum(dx,dy));
		if(nowresult<result)result=nowresult;
	}
	printf("%d\n",result);
	return 0;
}