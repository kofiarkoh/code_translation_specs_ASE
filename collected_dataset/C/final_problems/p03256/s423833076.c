#include <stdio.h>
#include <stdlib.h>
#define graph_valtype int
#define queue_valtype pair

typedef struct {
	int num;
	int ol;
}pair;

typedef struct {
	int a;
	int b;
}edge;

typedef struct q_node_sub{
	queue_valtype data;
	struct q_node_sub *next;
}q_node;

typedef struct {
	q_node *fst;
	q_node *last;
	int num;
}queue;

queue *make_queue(){
	queue *q = (queue *)malloc(sizeof(queue));
	q->fst = NULL;
	q->last = NULL;
	q->num = 0;
	return q;
}

int element_num_q(queue *q){
	return q->num;
}

void add_data_q(queue_valtype val, queue *q){
	q_node *last = (q_node *)malloc(sizeof(q_node));
	last->data = val;
	last->next = NULL;
	if(element_num_q(q) == 0){
		q->fst = last;
	}
	else{
		q->last->next = last;
	}
	q->last = last;
	q->num++;
}

queue_valtype take_data_q(queue *q){
	if(q->num == 0){
		printf("no data in the queue\n");
	}
	queue_valtype ans = q->fst->data;
	q_node *fst = q->fst;
	if(q->num == 1){
		q->fst = NULL;
		q->last = NULL;
	}
	else{
		q->fst = fst->next;
	}
	free(fst);
	q->num--;
	return ans;
}

queue_valtype look_data_q(queue *q){
	if(q->num == 0){
		printf("no data in the queue\n");
	}
	else{
		return q->fst->data;
	}
}
typedef struct graph_edge_sub graph_edge;

typedef struct {
	int num;
	int next_num;
	graph_edge *next;
	int prev_num;
}graph_vertex_sub;

struct graph_edge_sub{
	graph_vertex_sub *v;
	int w;
	graph_edge *next;
};

typedef struct graph_v_sub graph_vertex;

struct graph_v_sub{
	int num;
	graph_valtype val;
	int next_num;
	graph_vertex **next;
	int *next_weight;
	int prev_num;
	graph_vertex **prev;
	int *prev_weight;
};

typedef struct {
	int N;
	graph_vertex_sub **v_s;
	graph_vertex **v;
}graph;

//頂点数N, 各頂点の初期値ini_valのグラフを作る
graph *make_graph(int N, graph_valtype ini_val){
	int i;
	graph *g = (graph *)malloc(sizeof(graph));
	g->N = N;
	g->v_s = (graph_vertex_sub **)malloc(sizeof(graph_vertex_sub *) * N);
	g->v = (graph_vertex **)malloc(sizeof(graph_vertex *) * N);
	for(i = 0; i < N; i++){
		(g->v_s)[i] = (graph_vertex_sub *)malloc(sizeof(graph_vertex_sub));
		(g->v_s)[i]->num = i;
		(g->v_s)[i]->next_num = 0;
		(g->v_s)[i]->next = NULL;
		(g->v_s)[i]->prev_num = 0;
		(g->v)[i] = (graph_vertex *)malloc(sizeof(graph_vertex));
		(g->v)[i]->num = i;
		(g->v)[i]->val = ini_val;
		(g->v)[i]->next_num = 0;
		(g->v)[i]->next = NULL;
		(g->v)[i]->next_weight = NULL;
		(g->v)[i]->prev_num = 0;
		(g->v)[i]->prev = NULL;
		(g->v)[i]->prev_weight = NULL;
	}
	return g;
}

//グラフgの頂点aから頂点bに重みwの有向辺を張る (0 <= a, b <= N - 1)
void set_edge_graph(int a, int b, int w, graph *g){
	graph_edge *new1 = (graph_edge *)malloc(sizeof(graph_edge));
	new1->v = (g->v_s)[b];
	new1->w = w;
	new1->next = (g->v_s)[a]->next;
	(g->v_s)[a]->next = new1;
	(g->v_s)[a]->next_num++;
	(g->v_s)[b]->prev_num++;
}

//set_edge_graph後に呼び出す
void build_graph(graph *g){
	int i;
	graph_vertex_sub **v_s = g->v_s;
	graph_vertex **v = g->v;
	graph_vertex *nowv;
	graph_edge *nowe;
	for(i = 0; i < g->N; i++){
		v[i]->next = (graph_vertex **)malloc(sizeof(graph_vertex *) * v_s[i]->next_num);
		v[i]->next_weight = (int *)malloc(sizeof(int) * v_s[i]->next_num);
		v[i]->prev = (graph_vertex **)malloc(sizeof(graph_vertex *) * v_s[i]->prev_num);
		v[i]->prev_weight = (int *)malloc(sizeof(int) * v_s[i]->prev_num);
	}
	for(i = 0; i < g->N; i++){
		nowv = v[i];
		for(nowe = v_s[i]->next; nowe != NULL; nowe = nowe->next){
			(nowv->next)[nowv->next_num] = v[nowe->v->num];
			(nowv->next_weight)[nowv->next_num] = nowe->w;
			nowv->next_num++;
			(v[nowe->v->num]->prev)[v[nowe->v->num]->prev_num] = nowv;
			(v[nowe->v->num]->prev_weight)[v[nowe->v->num]->prev_num] = nowe->w;
			v[nowe->v->num]->prev_num++;
		}
	}
}

signed compare_edge(const void *x, const void *y){
	int sub = ((edge *)x)->a - ((edge *)y)->a;
	if(sub == 0){
		return ((edge *)x)->b - ((edge *)y)->b;
	}
	else{
		return sub;
	}
}

void set(edge e, graph *g){
	if(e.a == e.b){
		set_edge_graph(e.a, e.b, 1, g);
	}
	else{
		set_edge_graph(e.a, e.b, 1, g);
		set_edge_graph(e.b, e.a, 1, g);
	}
}

int main(){
	int N, M, a, b, i, j, first, ol;
	scanf("%d%d", &N, &M);
	char *s = (char *)malloc(sizeof(char) * N);
	scanf("%s", s);
	edge *es = (edge *)malloc(sizeof(edge) * M);
	for(i = 0; i < M; i++){
		scanf("%d%d", &es[i].a, &es[i].b);
		es[i].a--;
		es[i].b--;
	}
	qsort(es, M, sizeof(edge), compare_edge);
	graph *g = make_graph(N, 0);
	for(i = 1, j = 1; i < M; i++){
		if(es[i].a != es[i - 1].a || es[i].b != es[i - 1].b){
			es[j] = es[i];
			j++;
		}
	}
	for(i = 0; i < j; i++){
		set(es[i], g);
	}
	build_graph(g);
	int *is_used0 = (int *)malloc(sizeof(int) * N);
	int *is_used1 = (int *)malloc(sizeof(int) * N);
	for(i = 0; i < N; i++){
		is_used0[i] = -1;
		is_used1[i] = -1;
	}
	graph_vertex *nowv;
	pair p;
	queue *q = make_queue();
	for(first = 0; first < j; first++){
		if(s[es[first].a] == s[es[first].b]){
			continue;
		}
		if(is_used0[es[first].a] >= 0 || is_used1[es[first].b] >= 0){//ayashii
			continue;
		}
//		printf("first = %d\n", first);
		p.num = es[first].a;
		p.ol = 1;
//		add_data_q(p, q);
		is_used0[es[first].a] = first;
		p.num = es[first].b;
		p.ol = 1;
		add_data_q(p, q);
		while(element_num_q(q) > 0){
			p = take_data_q(q);
			nowv = g->v[p.num];
			ol = p.ol;
//			printf("(nowv->num, ol) = (%d, %d)\n", nowv->num, ol);
			if(ol == 0){
				if(nowv->num == es[first].a /*is_used0[nowv->num] == first*/){
					printf("Yes\n");
/*					printf("test1\n");
					for(i = 0; i < N; i++){
						printf("is_used0[%d] = %d\n", i, is_used0[i]);
					}
					for(i = 0; i < N; i++){
						printf("is_used1[%d] = %d\n", i, is_used1[i]);
					}
*/					return 0;
				}
				else{
					is_used0[nowv->num] = first;
				}
			}
			else{
			/*	if(is_used1[nowv->num] == first){
					printf("Yes\n");
					printf("test2\n");
					for(i = 0; i < N; i++){
						printf("is_used0[%d] = %d\n", i, is_used0[i]);
					}
					for(i = 0; i < N; i++){
						printf("is_used1[%d] = %d\n", i, is_used1[i]);
					}
					return 0;
				}
				else*/{
					is_used1[nowv->num] = first;
				}
			}
			for(i = 0; i < nowv->next_num; i++){
				if(ol == 0){
					if(s[nowv->num] != s[nowv->next[i]->num]){
						p.num = nowv->next[i]->num;
						p.ol = 1;
//						printf("(p.num, p.ol) = (%d, %d)\n", p.num, p.ol);
						add_data_q(p, q);
					}
				}
				else{
					if(s[nowv->num] == s[nowv->next[i]->num]){
						p.num = nowv->next[i]->num;
						p.ol = 0;
//						printf("(p.num, p.ol) = (%d, %d)\n", p.num, p.ol);
						add_data_q(p, q);
					}
				}
			}
		}
	}
	printf("No\n");
	return 0;
}