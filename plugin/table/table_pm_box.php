<?php

namespace pokemon;

use discuz_table;
use DB;

if(!defined('IN_DISCUZ')) {
    exit('Access Denied');
}

class table_pm_box extends discuz_table {

    public static function t() {
        static $_instance;
        if(!isset($_instance)) {
            $_instance = new self();
        }
        return $_instance;
    }

    public function __construct() {
        $this->_table = 'pm_box';
        $this->_pk = 'id';
        parent::__construct();
    }

    public function fetch_by_uid_box($uid, $box_index) {
        return DB::fetch_all('SELECT * FROM %t WHERE uid=%d AND box_index=%d ORDER BY slot ASC', [
            $this->_table, $uid, $box_index
        ]);
    }

    public function count_by_uid($uid) {
        return DB::result_first('SELECT COUNT(*) FROM %t WHERE uid=%d', [$this->_table, $uid]);
    }
}
