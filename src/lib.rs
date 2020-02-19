#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::float_cmp)]
#![allow(clippy::cast_lossless)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate pretty_assertions;

#[cfg_attr(test, macro_use)]
extern crate util;

mod _1_two_sum;
//
mod _2_add_two_numbers;
//
mod _3_longest_substring_without_repeating_characters;
//
mod _5_longest_palindromic_substring;
//
mod _6_zigzag_conversion;
//
mod _7_reverse_integer;
//
mod _8_string_to_integer;
//
mod _9_palindrome_number;
//
mod _11_container_with_most_water;
//
mod _12_integer_to_roman;
//
mod _13_roman_to_integer;
//
mod _14_longest_common_prefix;
//
mod _15_three_sum;
//
mod _16_3sum_closest;
//
mod _17_letter_combinations_of_a_phone_number;
//
mod _18_4sum;
//
mod _19_remove_nth_node_from_end_of_list;
//
mod _20_valid_parentheses;
//
mod _21_merge_two_sorted_lists;
//
mod _22_generate_parentheses;
//
mod _24_swap_nodes_in_pairs;
//
mod _26_remove_duplicates_from_sorted_array;
//
mod _27_remove_element;
//
mod _28_implement_str_str;
//
mod _29_divide_two_integers;
//
mod _31_next_permutation;
//
mod _33_search_in_rotated_sorted_array;
//
mod _34_find_first_and_last_position_of_elements_in_sorted_array;
//
mod _35_search_insert_position;
//
mod _36_valid_sudoku;
//
mod _38_count_and_say;
//
mod _39_combination_sum;
//
mod _43_multiply_strings;
//
mod _46_permutations;
//
mod _48_rotate_image;
//
mod _49_group_anagrams;
//
mod _50_pow_x_n;
//
mod _53_maximum_subarray;
//
mod _54_spiral_matrix;
//
mod _56_merge_intervals;
//
mod _59_spiral_matrix_2;
//
mod _62_unique_paths;
//
mod _64_minimum_path_sum;
//
mod _66_plus_one;
//
mod _67_add_binary;
//
mod _69_sqrt;
//
mod _70_climbing_stairs;
//
mod _71_simplify_path;
//
mod _73_set_matrix_zeroes;
//
mod _74_search_a_2d_matrix;
//
mod _75_sort_colors;
//
mod _78_subsets;
//
mod _79_word_search;
//
mod _83_remove_duplicates_from_sorted_list;
//
mod _88_merge_sorted_array;
//
mod _91_decode_ways;
//
mod _92_reverse_linked_list_2;
//
mod _93_restore_ip_addresses;
//
mod _94_binary_tree_inorder_traversal;
//
mod _95_unique_binary_search_trees_2;
//
mod _96_unique_binary_search_trees;
//
mod _98_validate_binary_search_tree;
//
mod _100_same_tree;
//
mod _101_symmetric_tree;
//
mod _102_binary_tree_level_order_traversal;
//
mod _103_binary_tree_zigzag_level_order_traversal;
//
mod _104_maximum_depth_of_binary_tree;
//
mod _105_construct_binary_tree_from_preorder_and_inorder_traversal;
//
mod _107_binary_tree_level_order_traversal_2;
//
mod _108_convert_sorted_array_binary_search_tree;
//
mod _109_convert_sorted_list_to_binary_search_tree;
//
mod _110_balanced_binary_tree;
//
mod _111_minimum_depth_of_binary_tree;
//
mod _112_path_sum;
//
mod _114_flatten_binary_tree_to_linked_list;
//
mod _118_pascal_triangle;
//
mod _119_pascal_triangle_2;
//
mod _121_best_time_to_buy_and_sell_stock;
//
mod _122_best_time_to_buy_and_sell_stock_2;
//
mod _125_valid_palindrome;
//
mod _127_word_ladder;
//
mod _131_palindrome_partitioning;
//
mod _134_gas_station;
//
mod _136_single_number;
//
mod _139_word_break;
//
mod _143_reorder_list;
//
mod _146_lru_cache;
//
mod _148_sort_list;
//
mod _151_reverse_words_in_a_string;
//
mod _152_maximum_product_subarray;
//
mod _153_find_minimum_in_rotated_sorted_array;
//
mod _155_min_stack;
//
mod _161_one_edit_distance;
//
mod _162_find_peak_element;
//
mod _166_fraction_to_recurring_decimal;
//
mod _167_two_sum_2;
//
mod _168_excel_sheet_column_title;
//
mod _169_majority_element;
//
mod _170_two_sum_3;
//
mod _171_excel_sheet_column_number;
//
mod _172_factorial_trailing_zeroes;
//
mod _173_binary_search_tree_iterator;
//
mod _179_largest_number;
//
mod _189_rotate_array;
//
mod _198_house_robber;
//
mod _199_binary_tree_right_side_view;
//
mod _200_number_of_islands;
//
mod _202_happy_number;
//
mod _203_remove_linked_list_elements;
//
mod _204_count_primes;
//
mod _205_isomorphic_strings;
//
mod _206_reverse_linked_list;
//
mod _207_course_schedule;
//
mod _208_implement_trie;
//
mod _209_minimum_size_subarray_sum;
//
mod _210_course_schedule_2;
//
mod _211_add_and_search_word_data_structure_design;
//
mod _215_kth_largest_element_in_an_array;
//
mod _217_contains_duplicate;
//
mod _219_contains_duplicate_2;
//
mod _221_maximal_square;
//
mod _222_count_complete_tree_nodes;
//
mod _225_implement_stack_using_queues;
//
mod _226_invert_binary_tree;
//
mod _227_basic_calculator_2;
//
mod _228_summary_ranges;
//
mod _231_power_of_two;
//
mod _232_implent_queue_using_stacks;
//
mod _234_palindrome_linked_list;
//
mod _238_product_of_array_except_self;
//
mod _242_valid_anagram;
//
mod _243_shortest_word_distance;
//
mod _244_shortest_word_distance_2;
//
mod _246_strobogrammantic_number;
//
mod _251_flatten_2d_vector;
//
mod _257_binary_tree_paths;
//
mod _258_add_digits;
//
mod _252_meeting_rooms;
//
mod _253_meeting_rooms_2;
//
mod _256_paint_house;
//
mod _263_ugly_number;
//
mod _266_palindrome_permutation;
//
mod _268_missing_number;
//
mod _270_closest_binary_search_tree_value;
//
mod _274_h_index;
//
mod _276_paint_fence;
//
mod _283_move_zeros;
//
mod _286_walls_and_gates;
//
mod _289_game_of_life;
//
mod _290_word_pattern;
//
mod _292_nim_game;
//
mod _293_flip_game;
//
mod _300_longest_increasing_subsequence;
//
mod _303_range_sum_query;
//
mod _304_range_sum_query_2d_immutable;
//
mod _309_best_time_to_buy_and_sell_stock_with_cooldown;
//
mod _311_sparse_matrix_multiplication;
//
mod _322_coin_change;
//
mod _328_odd_even_linked_list;
//
mod _332_reconstruct_itinerary;
//
mod _334_increasing_triplet_subsequence;
//
mod _339_nested_list_weight_sum;
//
mod _342_power_of_four;
//
mod _344_reverse_string;
//
mod _345_reverse_vowels_of_a_string;
//
mod _346_moving_average_from_data_stream;
//
mod _347_top_k_frequent_elements;
//
mod _348_design_tic_tac_toe;
//
mod _349_intersection_of_two_arrays;
//
mod _350_intersection_of_two_arrays_2;
//
mod _353_design_snake_game;
//
mod _355_design_twitter;
//
mod _359_logger_rate_limiter;
//
mod _326_power_of_three;
//
mod _362_design_hit_counter;
//
mod _364_nested_list_weight_sum_2;
//
mod _366_find_leaves_of_binary_tree;
//
mod _367_valid_perfect_square;
//
mod _371_sum_of_two_integers;
//
mod _378_kth_smallest_element_in_a_sorted_matrix;
//
mod _380_insert_delete_get_random_o1;
//
mod _383_ransom_note;
//
mod _387_first_unique_character_in_a_string;
//
mod _388_longest_absolute_file_path;
//
mod _389_find_the_difference;
//
mod _390_elimination_game;
//
mod _392_is_subsequence;
//
mod _393_utf8_validation;
//
mod _394_decode_string;
//
mod _395_longest_substring_with_at_least_k_repeating_characters;
//
mod _399_evaluate_division;
//
mod _401_binary_watch;
//
mod _404_sum_of_left_leaves;
//
mod _405_convert_a_number_to_hexadecimal;
//
mod _406_queue_reconstruction_by_height;
//
mod _408_valid_word_abbreviation;
//
mod _409_longest_palindrome;
//
mod _412_fizz_buzz;
//
mod _414_third_maximum_number;
//
mod _415_add_strings;
//
mod _416_partition_equal_subset_sum;
//
mod _417_pacific_atlantic_water_flow;
//
mod _419_battleships_in_a_board;
//
mod _422_valid_word_square;
//
mod _434_number_of_segments_in_a_string;
//
mod _437_path_sum_3;
//
mod _438_find_all_anagrams_in_a_string;
//
mod _441_arranging_coins;
//
mod _442_find_all_duplicates_in_an_array;
//
mod _443_string_compression;
//
mod _445_add_two_numbers_2;
//
mod _447_number_of_boomerangs;
//
mod _448_find_all_numbers_disappeared_in_an_array;
//
mod _453_minimum_moves_to_equal_array_elements;
//
mod _455_assign_cookies;
//
mod _456_132_pattern;
//
mod _459_repeated_substring_pattern;
//
mod _461_hamming_distance;
//
mod _463_island_perimeter;
//
mod _475_heaters;
//
mod _476_number_complement;
//
mod _482_license_key_formatting;
//
mod _485_max_consecutive_ones;
//
mod _490_the_maze;
//
mod _492_construct_the_rectangle;
//
mod _496_next_greater_element_1;
//
mod _498_diagonal_traverse;
//
mod _500_keyboard_row;
//
mod _501_find_mode_in_binary_search_tree;
//
mod _503_next_greater_element_2;
//
mod _504_base_7;
//
mod _506_relative_ranks;
//
mod _507_perfect_number;
//
mod _509_fibonacci_number;
//
mod _516_longest_palindromic_subsequence;
//
mod _518_coin_change_2;
//
mod _520_detect_captial;
//
mod _521_longest_uncommon_subsequence_1;
//
mod _523_continuous_subarray_sum;
//
mod _525_contiguous_array;
//
mod _528_random_pick_with_weight;
//
mod _529_minesweeper;
//
mod _530_minimum_absolute_difference_in_bst;
//
mod _532_k_diff_pairs_in_an_array;
//
mod _538_convert_bst_to_greater_tree;
//
mod _539_minimum_time_difference;
//
mod _541_reverse_string_2;
//
mod _543_diameter_of_binary_tree;
//
mod _544_output_contest_matches;
//
mod _545_boundary_of_binary_tree;
//
mod _547_friend_circles;
//
mod _551_student_attendance_record_1;
//
mod _556_next_greater_element_3;
//
mod _557_reverse_words_in_a_string_3;
//
mod _560_subarray_sum_equals_k;
//
mod _561_array_partition_1;
//
mod _563_binary_tree_tilt;
//
mod _566_reshape_the_matrix;
//
mod _572_subtree_of_another_tree;
//
mod _575_distribute_candies;
//
mod _581_shortest_unsorted_continuous_subarray;
//
mod _593_valid_square;
//
mod _594_longest_harmonious_subsequence;
//
mod _598_range_addition_2;
//
mod _599_minimum_index_sum_of_two_lists;
//
mod _604_design_compressed_string_iterator;
//
mod _605_can_place_flowers;
//
mod _606_construct_string_from_binary_tree;
//
mod _609_find_duplicate_file_in_system;
//
mod _611_valid_triangle_number;
//
mod _616_add_bold_tag_in_string;
//
mod _617_merge_two_binary_trees;
//
mod _621_task_scheduler;
//
mod _624_maximum_distance_in_arrays;
//
mod _628_maximum_product_of_three_numbers;
//
mod _633_sum_of_square_numbers;
//
mod _635_design_log_storage_system;
//
mod _636_exclusive_time_of_functions;
//
mod _637_average_of_levels_in_binary_tree;
//
mod _643_maximum_average_subarray_1;
//
mod _645_set_mismatch;
//
mod _647_palindromic_substrings;
//
mod _653_two_sum_4;
//
mod _654_maximum_binary_tree;
//
mod _655_print_binary_tree;
//
mod _657_robot_return_to_origin;
//
mod _661_image_smoother;
//
mod _665_non_decreasing_array;
//
mod _669_trim_a_binary_search_tree;
//
mod _671_second_minimum_node_in_a_binary_tree;
//
mod _674_longest_continuous_increasing_subsequence;
//
mod _680_valid_palindrome_2;
//
mod _682_baseball_game;
//
mod _686_repeated_string_match;
//
mod _687_longest_univalue_path;
//
mod _688_knight_probability_in_chessboard;
//
mod _692_top_k_frequent_words;
//
mod _693_binary_number_with_alternating_bits;
//
mod _694_number_of_distinct_islands;
//
mod _695_max_area_of_island;
//
mod _696_count_binary_substrings;
//
mod _697_degree_of_an_array;
//
mod _698_partition_to_k_equal_sum_subsets;
//
mod _700_search_in_a_binary_search_tree;
//
mod _701_insert_into_a_binary_search_tree;
//
mod _703_kth_largest_element_in_a_stream;
//
mod _704_binary_search;
//
mod _705_design_hash_set;
//
mod _706_design_hash_map;
//
mod _707_design_linked_list;
//
mod _709_to_lower_case;
//
mod _713_subarray_product_less_than_k;
//
mod _716_max_stack;
//
mod _717_1bit_and_2bit_characters;
//
mod _720_longest_word_in_dictionary;
//
mod _721_accounts_merge;
//
mod _723_candy_crush;
//
mod _724_find_pivot_index;
//
mod _728_self_dividing_numbers;
//
mod _733_flood_fill;
//
mod _734_sentence_similarity;
//
mod _735_asteroid_collision;
//
mod _739_daily_temperatures;
//
mod _740_delete_and_earn;
//
mod _742_closest_leaf_in_binary_tree;
//
mod _744_find_smallest_letter_greater_than_target;
//
mod _746_min_cost_climbing_stairs;
//
mod _747_largest_number_at_least_twice_of_others;
//
mod _748_shortest_completing_word;
//
mod _751_ip_to_cidr;
//
mod _754_reach_a_number;
//
mod _755_pour_water;
//
mod _756_pyramid_transition_matrix;
//
mod _758_bold_words_in_string;
//
mod _760_find_anagram_mappings;
//
mod _762_prime_number_of_set_bits_in_binary_representation;
//
mod _763_partition_labels;
//
mod _766_toeplitiz_matrix;
//
mod _771_jewels_and_stones;
//
mod _783_minimum_distance_between_bst_nodes;
//
mod _784_letter_case_permutation;
//
mod _785_is_graph_bipartite;
//
mod _787_cheapest_flights_within_k_stops;
//
mod _788_rotated_digits;
//
mod _796_rotate_string;
//
mod _797_all_paths_from_source_to_target;
//
mod _800_similar_rgb_color;
//
mod _804_unique_morse_code_words;
//
mod _806_number_of_lines_to_write_string;
//
mod _807_max_increase_to_keep_city_skyline;
//
mod _811_subdomain_visit_count;
//
mod _812_largest_triangle_area;
//
mod _814_binary_tree_pruning;
//
mod _819_most_common_word;
//
mod _821_shortest_distance_to_a_character;
//
mod _824_goat_latin;
//
mod _830_positions_of_large_groups;
//
mod _832_flipping_an_image;
//
mod _836_rectangle_overlap;
//
mod _840_magic_squares_in_grid;
//
mod _844_backspace_string_compare;
//
mod _849_maximize_distance_to_closest_person;
//
mod _852_peak_index_in_a_mountain_array;
//
mod _855_exam_room;
//
mod _859_buddy_strings;
//
mod _860_lemonade_change;
//
mod _866_prime_palindrome;
//
mod _867_transpose_matrix;
//
mod _868_binary_gap;
//
mod _872_leaf_similar_trees;
//
mod _874_walking_robot_simulation;
//
mod _875_koko_eating_bananas;
//
mod _876_middle_of_the_linked_list;
//
mod _883_projection_area_of_3d_shapes;
//
mod _884_uncommon_words_from_two_sentences;
//
mod _888_fair_candy_swap;
//
mod _890_find_and_replace_pattern;
//
mod _892_surface_area_of_3d_shapes;
//
mod _893_groups_of_special_equivalent_string;
//
mod _894_all_possible_full_binary_trees;
//
mod _896_monotonic_array;
//
mod _897_increasing_order_search_tree;
//
mod _905_sort_array_by_parity;
//
mod _908_smallest_range_1;
//
mod _909_snakes_and_ladders;
//
mod _914_x_of_a_kind_in_a_deck_of_cards;
//
mod _917_reverse_only_letters;
//
mod _922_sort_array_by_parity_2;
//
mod _925_long_pressed_name;
//
mod _929_unique_email_addresses;
//
mod _933_number_of_recent_calls;
//
mod _935_knight_dialer;
//
mod _937_reorder_log_files;
//
mod _938_range_sum_of_bst;
//
mod _939_minimum_area_rectangle;
//
mod _941_valid_mountain_array;
//
mod _942_di_string_match;
//
mod _944_delete_columns_to_make_sorted;
//
mod _945_minimum_increment_to_make_array_unique;
//
mod _949_largest_time_for_given_digits;
//
mod _950_reveal_cards_in_increasing_order;
//
mod _953_verifying_an_alien_dictionary;
//
mod _957_prison_cells_after_n_days;
//
mod _961_n_repeated_element_in_size_2n_array;
//
mod _965_univalued_binary_tree;
//
mod _970_powerful_integers;
//
mod _973_k_closest_points_to_origin;
//
mod _974_subarray_sums_divisible_by_k;
//
mod _976_largest_perimeter_triangle;
//
mod _977_squares_of_a_sorted_array;
//
mod _981_time_based_key_value_store;
//
mod _985_sum_of_even_numbers_after_queries;
//
mod _986_interval_list_intersections;
//
mod _987_vertical_order_traversal_of_a_binary_tree;
//
mod _989_add_to_array_form_of_integer;
//
mod _991_broken_calculator;
//
mod _994_rotting_oranges;
//
mod _999_available_captures_for_rook;
//
mod _1002_find_common_characters;
//
mod _1005_maximize_sum_of_array_after_k_negations;
//
mod _1007_minimum_domino_rotations_for_equal_row;
//
mod _1008_construct_binary_search_tree_from_preorder_traversal;
//
mod _1009_complement_of_base_10_integer;
//
mod _1010_pairs_of_songs_with_total_durations_divisible_by_60;
//
mod _1011_capacity_to_ship_packages_within_d_days;
//
mod _1018_binary_prefix_divisible_by_5;
//
mod _1013_partition_array_into_three_parts_with_equal_sum;
//
mod _1021_remove_outermost_parentheses;
//
mod _1022_sum_root_to_leaf_binary_number;
//
mod _1025_divisor_game;
//
mod _1027_longest_arithmetic_sequence;
//
mod _1029_two_city_scheduling;
//
mod _1030_matrix_cells_in_distance_order;
//
mod _1031_maximum_sum_of_two_non_overlapping_subarrays;
//
mod _1033_moving_stones_until_consecutive;
//
mod _1037_valid_boomerang;
//
mod _1038_binary_search_tree_to_greater_sum_tree;
//
mod _1041_robot_bounded_in_circle;
//
mod _1042_flower_planting_with_no_adjacent;
//
mod _1046_last_stone_weight;
//
mod _1047_remove_all_adjacent_duplicates_in_string;
//
mod _1048_longest_string_chain;
//
mod _1051_height_checker;
//
mod _1052_grumpy_bookstore_owner;
//
mod _1055_shortest_way_to_form_string;
//
mod _1056_confusing_number;
//
mod _1057_campus_bikes;
//
mod _1060_missing_element_in_sorted_array;
//
mod _1064_fixed_point;
//
mod _1065_index_pairs_of_a_string;
//
mod _1071_greatest_common_divisor_of_strings;
//
mod _1078_occurrences_after_bigram;
//
mod _1079_letter_tile_possibilities;
//
mod _1085_sum_of_digits_in_the_minmum_number;
//
mod _1089_duplicate_zeros;
//
mod _1190_reverse_substrings_between_each_pair_of_parentheses;
//
mod _1091_shortest_path_in_binary_matrix;
//
mod _1103_distribute_candies_to_people;
//
mod _1108_defanging_an_ip_address;
//
mod _1110_delete_nodes_and_return_forest;
//
mod _1118_number_of_days_in_a_month;
//
mod _1119_remove_vowels_from_a_string;
//
mod _1122_relative_sort_array;
//
mod _1128_number_of_equivalent_domino_pairs;
//
mod _1130_minimum_cost_tree_from_leaf_values;
//
mod _1133_largest_unique_number;
//
mod _1134_armstrong_number;
//
mod _1137_n_th_tribonacci_number;
//
mod _1150_check_if_a_number_is_majority_element_in_a_sorted_array;
//
mod _1154_day_of_the_year;
//
mod _1160_find_words_that_can_be_formed_by_characters;
//
mod _1165_single_row_keyboard;
//
mod _1166_design_file_system;
//
mod _1167_minimum_cost_to_connect_sticks;
//
mod _1170_compare_strings_by_frequency_of_the_smallest_character;
//
mod _1175_prime_arrangements;
//
mod _1176_diet_plan_performance;
//
mod _1177_can_make_palindrome_from_substring;
//
mod _1180_count_substring_with_only_one_distinct_letter;
//
mod _1181_before_and_after_puzzle;
//
mod _1184_distance_between_bus_stops;
//
mod _1185_day_of_the_week;
//
mod _1189_maximum_number_of_balloons;
//
mod _1196_how_many_apples_can_you_put_into_the_basket;
//
mod _1197_minimum_knight_moves;
//
mod _1197_minimum_knight_moves_math;
//
mod _1198_find_smallest_common_element_in_all_rows;
//
mod _1200_minimum_absolute_difference;
//
mod _1207_unique_number_of_occurrences;
//
mod _1209_remove_all_adjacent_duplicates_in_string_2;
//
mod _1213_intersection_of_three_sorted_arrays;
//
mod _1217_play_with_chips;
//
mod _1221_split_a_string_in_balanced_strings;
//
mod _1223_dice_roll_simulation;
//
mod _1228_missing_number_in_arithmetic_progression;
//
mod _1232_check_if_it_is_a_straight_line;
//
mod _1243_array_transformation;
//
mod _1249_minimum_remove_to_make_valid_parentheses;
//
mod _1252_cells_with_odd_values_in_a_matrix;
//
mod _1260_shift_2d_grid;
//
mod _1266_minimum_time_visition_all_points;
//
mod _1268_search_suggestions_system;
//
mod _1271_hexspeak;
//
mod _1275_find_winner_on_a_tic_tac_toe_game;
//
mod _1281_subtract_the_product_and_sum_of_digits_of_an_integer;
//
mod _1282_group_the_people_given_the_group_size_they_belong_to;
//
mod _1287_element_appearing_more_than_25_in_sorted_array;
//
mod _1290_convert_binary_number_in_a_linked_list_to_integer;
//
mod _1295_find_numbers_with_even_number_of_digits;
//
mod _1299_replace_elements_with_greatest_element_on_right_side;
//
mod _1302_deepest_leaves_sum;
//
mod _1304_find_n_unique_integers_sum_up_to_zero;
//
mod _1305_all_elements_in_two_binary_search_tree;
//
mod _1309_decrypt_string_from_alphabet_to_integer_mapping;
//
mod _1317_convert_integer_to_the_sum_of_two_no_zero_integers;
//
mod _1313_decompres_run_length_encoded_list;
//
mod _1315_sum_of_nodes_with_even_valued_grandparent;
//
mod _1323_maximum_69_number;
//
mod _1325_delete_leaves_with_a_given_value;
//
mod _1329_sort_the_matrix_diagonally;
//
mod _1331_rank_transform_of_an_array;
//
mod _1332_remove_palindromic_subsequences;
//
mod _1337_the_k_weakest_rows_in_a_matrix;
//
mod _1342_number_of_steps_to_reduce_a_number_to_zero;
//
mod _1346_check_if_n_and_its_double_exist;
//
mod _1347_minimum_number_of_steps_to_make_two_strings_anagram;
//
mod _1351_count_negative_numbers_in_a_sorted_matrix;
