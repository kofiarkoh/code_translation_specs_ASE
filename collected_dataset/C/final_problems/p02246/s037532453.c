#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

/*----------       定数定義       ----------*/

#define BOARD_SIZE             4   // パズルの行数（列数）
#define PANEL_NUM             16   // マスの総数
#define SPACE                  0   // 空白マス
#define MAX_LIMIT             80   // 最大移動手数 (15パズルは80手以内に必ず解ける)
#define SEARCH_FAILURE        -1   // 探索失敗
#define ALREADY_PASSED         2   // 前回通った道
#define MOVE_DIRECTION_NUM     4   // 移動方向の数
#define INITIAL_PREV_DIRECTION 6   // 初期盤面における過去の移動方向


/*----------      構造体宣言      ----------*/

/*
 *  パズル盤面の情報をもつ構造体
 */
typedef struct {
    int field[PANEL_NUM];        // パズルの配置
    int space_position;          // 空白の位置
    int sum_manhattan_distance;  // マンハッタン距離の総和
} Puzzle;


/*----------   プロトタイプ宣言   ----------*/

int IterativeDeepening(Puzzle* puzzle, int** manhattan_distance_table);
bool DepthFirstSearch(int depth, int** manhattan_distance_table, int limit, Puzzle* current_state, int prev_direction);
int** CreateManhattanDistanceTable();
int CalcSumManhattanDistance(Puzzle* puzzle, int** manhattan_distance_table);
void swap(int*a,int*b);

/*----------      メイン処理      ----------*/

int main() {
    int** manhattan_distance_table = CreateManhattanDistanceTable();
    if(!manhattan_distance_table){
        printf("malloc error\n");
        return EXIT_FAILURE;
    }

    Puzzle start;

    for(int i = 0; i < PANEL_NUM; i++){
        scanf("%d",&start.field[i]);
        if (start.field[i] == SPACE) {
            start.space_position = i;
        }
    }
    start.sum_manhattan_distance = CalcSumManhattanDistance(&start, manhattan_distance_table);

    int min_moves_num = IterativeDeepening(&start, manhattan_distance_table); // 反復深化で最短手数を求める

    if(min_moves_num != SEARCH_FAILURE){
        printf("%d\n", min_moves_num);
    }
    else{
        printf("探索失敗\n");
    }

    for(int i = 0.; i < PANEL_NUM; i++){
        free(manhattan_distance_table[i]);
    }
    free(manhattan_distance_table);

    return EXIT_SUCCESS;
}


/*----------       関数定義       ----------*/

/*
 *  反復深化をすることでパズルの最短手数を求める
 *
 *  戻り値：
 *      最短手数
 *      80手以上かかる場合  SEARCH_FAILURE
 */
int IterativeDeepening(Puzzle* puzzle, int** manhattan_distance) {
    Puzzle state;
    for(int limit = puzzle->sum_manhattan_distance; limit <= MAX_LIMIT; limit++){
        state = *puzzle;
		int depth = 0;
        if(DepthFirstSearch(depth, manhattan_distance, limit, &state, INITIAL_PREV_DIRECTION)){
            return limit;
        }
    }
    return SEARCH_FAILURE;
}

/*
 *  深さ優先探索をしながらパズルを動かす
 *
 *  戻り値：
 *      探索に成功したら true、失敗すれば false
 */
bool DepthFirstSearch(int depth, int** manhattan_distance_table, int limit, Puzzle* current_state, int prev_direction) {
    if(current_state->sum_manhattan_distance == 0){
        return true; // ゴール盤面と一致したら探索成功
    }

    if(current_state->sum_manhattan_distance + depth > limit){
        return false; // コストがlimitを超えたら探索失敗
    }

    int current_space_pos_x = current_state->space_position / BOARD_SIZE;
    int current_space_pos_y = current_state->space_position % BOARD_SIZE;

    // 下、右、上、左の移動方向を定義
    const int move_direction_x[MOVE_DIRECTION_NUM] = { 0, 1, 0, -1 };
    const int move_direction_y[MOVE_DIRECTION_NUM] = { 1, 0, -1, 0 };

    for(int direction = 0; direction < MOVE_DIRECTION_NUM; direction++){
        int moved_space_pos_x = current_space_pos_x + move_direction_x[direction];
        int moved_space_pos_y = current_space_pos_y + move_direction_y[direction];

        // 移動後の空白の位置がパズルの枠から出ている場合は次へ
        if(moved_space_pos_x < 0 || moved_space_pos_x >= BOARD_SIZE || moved_space_pos_y < 0 || moved_space_pos_y >= BOARD_SIZE){
            continue;
        }

        // 前回と同じ位置の場合は次へ
        if(abs(prev_direction - direction) == ALREADY_PASSED){
            continue;
        }

        Puzzle next_state = *current_state;
        int next_state_space_position = moved_space_pos_x * BOARD_SIZE + moved_space_pos_y;

        // 空白の位置を移動
        swap(&next_state.field[next_state_space_position], &next_state.field[current_state->space_position]);
        next_state.space_position = next_state_space_position;

        next_state.sum_manhattan_distance = CalcSumManhattanDistance(&next_state, manhattan_distance_table);

        // 再帰呼び出しで深さ優先探索
        if(DepthFirstSearch(depth + 1, manhattan_distance_table, limit, &next_state, direction)){
            return true;
        }
    }
    return false;
}

/*
 *  値を入れ替える
 *
 *  戻り値:
 *      なし
 */
void swap(int*a,int*b) {
    int tmp = *a;
    *a = *b;
    *b = tmp;
}

/*
 *  マンハッタン距離のテーブルを作成する
 *
 *  戻り値：
 *      マンハッタン距離テーブル
 *      メモリ確保失敗した場合  NULL
 */
int** CreateManhattanDistanceTable() {
    int** manhattan_distance_table = (int**) malloc(sizeof(int*) * PANEL_NUM);
    if(!manhattan_distance_table){
        return NULL;
    }

    int malloc_err = 0;
    for(int i = 0; i < PANEL_NUM; i++){
        manhattan_distance_table[i] = (int*) malloc(sizeof(int) * PANEL_NUM);
        if(!manhattan_distance_table[i]){
            malloc_err = i;
            break;
        }
    }

    if(malloc_err){
    	printf("%d", malloc_err);
        for(int i = 0; i < malloc_err; i++){
            free(manhattan_distance_table[i]);
        }
        free(manhattan_distance_table);
        return NULL;
    }

    for(int i = 0; i < PANEL_NUM; ++i){
        for(int j = 0; j < PANEL_NUM; ++j){
            manhattan_distance_table[i][j] = abs(i / BOARD_SIZE - j / BOARD_SIZE) + abs(i % BOARD_SIZE - j % BOARD_SIZE);
        }
    }
    return manhattan_distance_table;
}

/*
 *  パズルのそれぞれのパネルのマンハッタン距離を計算する
 *
 *  戻り値：
 *      マンハッタン距離の総和
 */
int CalcSumManhattanDistance(Puzzle* puzzle, int** manhattan_distance) {
    int res = 0;
    for(int i = 0; i < PANEL_NUM; i++){
        if(puzzle->field[i] == SPACE){
            continue;
        }
        res += manhattan_distance[i][puzzle->field[i] - 1];
    }
    return res;
}

